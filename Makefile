format: 
	cargo fmt

lint:
	cargo clippy -- -D warnings

test: 
	cargo test

run:
	cargo run

all: format lint test run

release:
	cargo build --release

transform:
	cargo run transform  "drink.db" "https://raw.githubusercontent.com/fivethirtyeight/data/master/alcohol-consumption/drinks.csv"

read_all:
	cargo run read

create:
	cargo create "Test Country1" 100 150 200 2.5

update:
	cargo run update "Test Country" 10

delete:
	cargo run delete "Test Country"

general:
	cargo run general "UPDATE drink SET beer_servings = 100 WHERE country = 'USA';"

