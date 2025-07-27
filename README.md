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
pyfindgrep.findgrep("path/to/files", find_patterns=[".*py$"], grep_patterns=["TODO"])

# filter files by file content pattern (i.e. similar to `grep` or `ripgrep`)
pyfindgrep.findgrep("path/to/files", grep_patterns=["TODO"])
```

## Development

Requires `maturin` and `uv` is recommended for managing Python tasks.

See `Makefile` for commands.
