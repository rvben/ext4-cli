.PHONY: build test fixtures clean install

build:
	cargo build

test:
	cargo nextest run

fixtures:
	bash tests/create_fixtures.sh

clean:
	cargo clean
	rm -f tests/fixtures/*.img

install:
	cargo install --path .
