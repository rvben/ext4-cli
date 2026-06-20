.PHONY: build test lint fixtures check clean install release-patch release-minor release-major

build:
	cargo build

test:
	cargo nextest run

lint:
	cargo fmt --check
	cargo clippy --all-targets -- -D warnings

fixtures:
	bash tests/create_fixtures.sh

check: lint fixtures test

clean:
	cargo clean
	rm -f tests/fixtures/*.img

install:
	cargo install --path .

release-patch:
	vership bump patch

release-minor:
	vership bump minor

release-major:
	vership bump major
