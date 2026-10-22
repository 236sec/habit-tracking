.PHONY: run check test format migrate-add migrate-run prepare

run:
	cargo run

check:
	cargo fmt --all -- --check
	cargo clippy --all-targets -- -D warnings

test:
	cargo test

format:
	cargo fmt --all

migrate-add:
	@test -n "$(name)" || (echo "Usage: make migrate-add name=create_habits_table"; exit 1)
	sqlx migrate add -r $(name)

migrate-run:
	sqlx migrate run

prepare:
	cargo sqlx prepare --workspace
