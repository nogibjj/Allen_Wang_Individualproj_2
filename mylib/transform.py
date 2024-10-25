import requests
import sqlite3
import csv
from io import StringIO

def csv_to_db(db_file, url):
    try:
        conn = sqlite3.connect(db_file)
        cursor = conn.cursor()
        cursor.execute('''
            SELECT name FROM sqlite_master WHERE type='table' AND name='drink';
        ''')
        table_exists = cursor.fetchone()

        if table_exists:
            print(f"The table 'drink' already exists in {db_file}. Skipping data insertion.")
            conn.close()
            return
        response = requests.get(url)
        if response.status_code != 200:
            raise Exception(f"Failed to fetch data from {url}. Status code: {response.status_code}")

        csv_data = response.text
        cursor.execute('''
            CREATE TABLE IF NOT EXISTS drink (
                country TEXT,
                beer_servings INTEGER,
                spirit_servings INTEGER,
                wine_servings INTEGER,
                total_litres_of_pure_alcohol REAL
            );
        ''')
        conn.commit()

        csv_reader = csv.reader(StringIO(csv_data))
        next(csv_reader)
        for row in csv_reader:
            if len(row) == 5:
                cursor.execute('''
                    INSERT INTO drink (country, beer_servings, spirit_servings, wine_servings, total_litres_of_pure_alcohol)
                    VALUES (?, ?, ?, ?, ?);
                ''', row)
                print(row)
        conn.commit()
        conn.close()

        print(f"Data from {url} successfully inserted into {db_file}")

    except Exception as e:
        print(f"An error occurred: {e}")

# Usage
# csv_to_db("drink.db", "https://raw.githubusercontent.com/fivethirtyeight/data/master/alcohol-consumption/drinks.csv")
