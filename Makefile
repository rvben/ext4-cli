.PHONY: build test lint fixtures clean install

build:
	cargo build

test:
	cargo nextest run

lint:
	cargo clippy --all-targets -- -D warnings

fixtures:
	bash tests/create_fixtures.sh

clean:
	cargo clean
	rm -f tests/fixtures/*.img

install:
	cargo install --path .
