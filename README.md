# pyfindgrep

Run `fd` and `ripgrep` from Python.

Inspired by `fd` and uses `ripgrep` crates.

## Usage

```python
import pyfindgrep

# list files, similar to `os.walk`
pyfindgrep.findgrep("path/to/files")

# run in parallel
pyfindgrep.findgrep("path/to/files", parallel=True)

# filter files by filename pattern (i.e. similar to `find` or `fd`)
pyfindgrep.findgrep("path/to/files", patterns=[".*py$"], content_patterns=["TODO"])

# filter files by file content pattern (i.e. similar to `grep` or `ripgrep`)
pyfindgrep.findgrep("path/to/files", content_patterns=["TODO"])
```

## Development

Requires `maturin` and `uv` is recommended for managing Python tasks.

Build with `maturin develop`, test with `cargo test`, `cargo test --workspace`, and/or `uv run pytest`
