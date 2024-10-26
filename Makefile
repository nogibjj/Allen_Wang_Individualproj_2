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
	cargo create "Test Country" 100 150 200 2.5

update:
	cargo run update "USA" 10

delete:
	cargo run delete "Togo"


