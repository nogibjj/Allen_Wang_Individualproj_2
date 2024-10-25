use allen_crud::{csv_to_db, create_row, read_all};


fn main() {
    match csv_to_db("drink.db", "https://raw.githubusercontent.com/fivethirtyeight/data/master/alcohol-consumption/drinks.csv") {
        Ok(_) => println!("CSV data loaded successfully."),
        Err(e) => eprintln!("Error: {}", e),
    }

    // Insert example
    match create_row("Test Country", 100, 200, 300, 4.5) {
        Ok(id) => println!("Row inserted with ID: {}", id),
        Err(e) => eprintln!("Error inserting row: {}", e),
    }

    // Read all rows
    match read_all() {
        Ok(rows) => {
            for row in rows {
                println!("{:?}", row);
            }
        }
        Err(e) => eprintln!("Error reading rows: {}", e),
    }
}
