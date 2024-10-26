use allen_crud::{create_row, csv_to_db, delete_row, general, read_all, update_row};
use std::env;
use std::error::Error;

fn handle_arguments() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!(
            "Usage: {} [create | read | update | delete | transform | general]",
            args[0]
        );
        return Ok(());
    }

    let action = &args[1];
    match action.as_str() {
        "create" => {
            if args.len() < 6 {
                println!("Usage: {} create [country] [beer_servings] [spirit_servings] [wine_servings] [total_litres_of_pure_alcohol]", args[0]);
                return Ok(());
            }
            let country = &args[2];
            let beer_servings: i32 = args[3].parse()?;
            let spirit_servings: i32 = args[4].parse()?;
            let wine_servings: i32 = args[5].parse()?;
            let total_litres_of_pure_alcohol: f32 = args[6].parse()?;
            match create_row(
                country,
                beer_servings,
                spirit_servings,
                wine_servings,
                total_litres_of_pure_alcohol.into(),
            ) {
                Ok(_) => println!("Row created successfully!"),
                Err(e) => eprintln!("Failed to create row: {}", e),
            }
        }
        "read" => match read_all() {
            Ok(rows) => {
                for row in rows {
                    println!("{:?}", row);
                }
            }
            Err(e) => eprintln!("Error reading rows: {}", e),
        },
        "update" => {
            if args.len() < 4 {
                println!("Usage: {} update [country] [beer_servings]", args[0]);
                return Ok(());
            }
            let country = &args[2];
            let beer_servings: i32 = args[3].parse()?;
            match update_row(country, beer_servings) {
                Ok(_) => println!("Row updated successfully!"),
                Err(e) => eprintln!("Failed to update row: {}", e),
            }
        }
        "delete" => {
            if args.len() < 3 {
                println!("Usage: {} delete [country]", args[0]);
                return Ok(());
            }
            let country = &args[2];
            match delete_row(country) {
                Ok(_) => println!("Row deleted successfully!"),
                Err(e) => eprintln!("Failed to delete row: {}", e),
            }
        }
        "transform" => {
            if args.len() < 4 {
                println!("Usage: {} transform [db_file] [url]", args[0]);
                return Ok(());
            }
            let db_file = &args[2];
            let url = &args[3];
            csv_to_db(db_file, url)?;
        }
        "general" => {
            if args.len() < 3 {
                println!("Usage: {} general [SQL query]", args[0]);
                return Ok(());
            }
            let query = &args[2];
            println!("{:?}", query);
            match general(query) {
                Ok(Some(result)) => {
                    for row in result {
                        println!("{:?}", row);
                    }
                }
                Ok(None) => {
                    println!("No results found for the query.");
                }
                Err(e) => {
                    eprintln!("Error executing query: {}", e);
                }
            }
        }
        _ => {
            println!("Invalid action. Use 'create', 'read', 'update', 'delete', 'transform', or 'general'.");
        }
    }
    Ok(())
}

fn main() {
    if let Err(e) = handle_arguments() {
        eprintln!("Error: {}", e);
    }
}
