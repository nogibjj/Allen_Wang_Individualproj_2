install: 
	pip install --upgrade pip && pip install -r requirements.txt 

format: 
	black *.py

lint:
	pylint --disable=R,C --ignore-patterns=test_.*?py $(wildcard *.py)

test: 
	python -m pytest -cov=main test_main.py

all: install format lint test

transform:
	python main.py transform  "drink.db" "https://raw.githubusercontent.com/fivethirtyeight/data/master/alcohol-consumption/drinks.csv"

query1:
	python main.py general "INSERT INTO drink(country,beer_servings,spirit_servings,wine_servings, total_litres_of_pure_alcohol) VALUES('USC', 10,100,1000,0.1) "

query2:
	python main.py general "UPDATE drink SET  total_litres_of_pure_alcohol  = -0.1 WHERE country = 'USA'"



