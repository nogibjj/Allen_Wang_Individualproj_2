import sqlite3


def create_row(country,beer_servings,spirit_servings,wine_servings, total_litres_of_pure_alcohol):
    try:
        conn = sqlite3.connect("drink.db")
        sql = '''INSERT INTO drink(country,beer_servings,spirit_servings,wine_servings, total_litres_of_pure_alcohol) VALUES(?, ?,?,?,?)'''
        cursor = conn.cursor()
        cursor.execute(sql, (country,beer_servings,spirit_servings,wine_servings, total_litres_of_pure_alcohol))
        conn.commit()
        conn.close()
        return cursor.lastrowid
    except sqlite3.Error as e:
        print(e)



def read_all():
    try:
        conn = sqlite3.connect("drink.db")
        sql = '''SELECT * FROM drink'''
        cursor = conn.cursor()
        cursor.execute(sql)
        rows = cursor.fetchall()
        conn.close()
        return rows
    except sqlite3.Error as e:
        print(e)


def update_row(country, beer_servings):
    try:
        conn = sqlite3.connect("drink.db")
        sql = '''UPDATE drink SET beer_servings = ? WHERE country = ?'''
        cursor = conn.cursor()
        cursor.execute(sql,  (beer_servings,country))
        conn.commit()
        conn.close()
    except sqlite3.Error as e:
        print(e)

def delete_row(country):
    try:
        conn = sqlite3.connect("drink.db")
        sql = '''DELETE FROM drink WHERE country = ?'''
        cursor = conn.cursor()
        cursor.execute(sql, (country,))
        conn.commit()
        conn.close()
    except sqlite3.Error as e:
        print(e)


def general(query):
    try:
        conn = sqlite3.connect("drink.db")
        cursor = conn.cursor()
        cursor.execute(query)
        if query.strip().lower().startswith("select"):
            results = cursor.fetchall()
            conn.close()
            return results
        conn.commit()
        conn.close()
    except sqlite3.Error as e:
        print(e)