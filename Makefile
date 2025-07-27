build:
	maturin develop

format:
	cargo fmt
	uv tool run ruff format python

test:
	cargo test --workspace
	uv run pytest

check:
	uv tool run ruff check --select E python
	cargo clippy --workspace --all-targets --all-features -- -D warnings
