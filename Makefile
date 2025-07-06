build:
	maturin develop

format:
	cargo fmt
	uv tool run ruff format python

pytest:
	uv run pytest

test:
	cargo test

test_all:
	cargo test --workspace
	uv run pytest
