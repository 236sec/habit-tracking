.PHONY: check lint format format-check typecheck test

check: format-check lint typecheck

format-check:
	cd backend && cargo fmt --all -- --check
	cd frontend && npm run format:check

lint:
	cd backend && cargo clippy --all-targets -- -D warnings
	cd frontend && npm run lint

typecheck:
	cd frontend && npm run typecheck

format:
	cd backend && cargo fmt --all
	cd frontend && npm run format

test:
	cd backend && cargo test
