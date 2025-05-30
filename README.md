# o3rg

A wrapper around ripgrep (and ignore) for grep functionality in Python with a simple interface. No need for shelling out, parsing output and insuring command line dependencies are installed.

## Installation

```bash
pip install o3rg
```

## Usage

### Python

```python
from o3rg import search, search_dir

# Search in a single file
matches = search("path/to/file.txt", r"pattern")
for match in matches:
    print(f"Line {match.line}: {match.match_result}")

# Search in a directory
matches = search_dir("path/to/dir", r"pattern", hidden=False)
for match, filepath in matches:
    print(f"{filepath} - Line {match.line}: {match.match_result}")
```

## Configuration

- `hidden`: When searching directories, controls whether hidden files are included
  - `true`: Ignore hidden files (default)
  - `false`: Include hidden files

## Development

### Prerequisites

- Rust 1.54 or later
- Python 3.7 or later (for Python bindings)
- maturin (for building Python package)

### Building

```bash
cargo build --release
```

For Python package:
```bash
maturin build --release
```

### Testing

```bash
cargo test
```

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request. 

## Notes

This is also a small project I've been testing out what coding with AI is like. So far its written most of the tests and READMEs.
