use rusqlite::{params, Connection, Result};
use csv::ReaderBuilder;
use std::io::Cursor;

type DrinkData = Vec<(String, i32, i32, i32, f64)>;

pub fn csv_to_db(db_file: &str, url: &str) -> Result<(), Box<dyn std::error::Error>> {
    let conn = Connection::open(db_file)?;

    // Check if the table 'drink' already exists
    let mut stmt = conn.prepare(
        "SELECT name FROM sqlite_master WHERE type='table' AND name='drink';",
    )?;
    let table_exists = stmt.exists([])?;

    if table_exists {
        println!("The table 'drink' already exists in {}. Skipping data insertion.", db_file);
        return Ok(());
    }

    // Fetch CSV data from the URL
    let response = reqwest::blocking::get(url)?;
    if !response.status().is_success() {
        return Err(format!("Failed to fetch data from {}. Status code: {}", url, response.status()).into());
    }
    let csv_data = response.text()?;

    // Create the 'drink' table if it doesn't exist
    conn.execute(
        "CREATE TABLE IF NOT EXISTS drink (
            country TEXT,
            beer_servings INTEGER,
            spirit_servings INTEGER,
            wine_servings INTEGER,
            total_litres_of_pure_alcohol REAL
        );",
        [],
    )?;

    // Use a CSV reader to parse the data and insert it into the database
    let mut csv_reader = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(Cursor::new(csv_data));

    for result in csv_reader.records() {
        let record = result?;
        if record.len() == 5 {
            conn.execute(
                "INSERT INTO drink (country, beer_servings, spirit_servings, wine_servings, total_litres_of_pure_alcohol)
                 VALUES (?, ?, ?, ?, ?);",
                params![
                    record.get(0).unwrap(),
                    record.get(1).unwrap(),
                    record.get(2).unwrap(),
                    record.get(3).unwrap(),
                    record.get(4).unwrap(),
                ],
            )?;
            println!("{:?}", record);
        }
    }

    println!("Data from {} successfully inserted into {}", url, db_file);
    Ok(())
}


pub fn create_row(country: &str, beer_servings: i32, spirit_servings: i32, wine_servings: i32, total_litres_of_pure_alcohol: f64) -> Result<u64> {
    let conn = Connection::open("drink.db")?;
    let sql = "INSERT INTO drink (country, beer_servings, spirit_servings, wine_servings, total_litres_of_pure_alcohol) VALUES (?, ?, ?, ?, ?)";
    
    conn.execute(
        sql,
        params![country, beer_servings, spirit_servings, wine_servings, total_litres_of_pure_alcohol],
    )?;

    Ok(conn.last_insert_rowid() as u64)
}

pub fn read_all() -> Result<DrinkData> {
    let conn = Connection::open("drink.db")?;
    let mut stmt = conn.prepare("SELECT * FROM drink")?;
    let rows = stmt.query_map([], |row| {
        Ok((
            row.get(0)?,
            row.get(1)?,
            row.get(2)?,
            row.get(3)?,
            row.get(4)?
        ))
    })?;

    let mut results = Vec::new();
    for row in rows {
        results.push(row?);
    }

    Ok(results)
}

pub fn update_row(country: &str, beer_servings: i32) -> Result<()> {
    let conn = Connection::open("drink.db")?;
    let sql = "UPDATE drink SET beer_servings = ? WHERE country = ?";
    conn.execute(sql, params![beer_servings, country])?;
    Ok(())
}

pub fn delete_row(country: &str) -> Result<()> {
    let conn = Connection::open("drink.db")?;
    let sql = "DELETE FROM drink WHERE country = ?";
    conn.execute(sql, params![country])?;
    Ok(())
}

pub fn general(query: &str) ->  Result<Option<DrinkData>> {
    let conn = Connection::open("drink.db")?;
    let mut stmt = conn.prepare(query)?;

    if query.trim().to_lowercase().starts_with("select") {
        let rows = stmt.query_map([], |row| {
            Ok((
                row.get(0)?,
                row.get(1)?,
                row.get(2)?,
                row.get(3)?,
                row.get(4)?
            ))
        })?;

        let mut results = Vec::new();
        for row in rows {
            results.push(row?);
        }

        Ok(Some(results))
    } else {
        conn.execute(query, [])?;
        Ok(None)
    }
}
