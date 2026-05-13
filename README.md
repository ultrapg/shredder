# shredder
Securely destroy files by overwriting them multiple times with random data

## Features

- Configurable overwrite passes (default: 3)
- 64 KB buffer for efficient processing of large files
- `sync_all()` after each pass to force hardware flush
- Metadata destruction by renaming to a random string before deletion
- Clean error handling

## Build

Requires [Rust](https://www.rust-lang.org/tools/install) (edition 2021).

```bash
git clone https://github.com/your-user/shredder.git
cd shredder
cargo build --release
```

## Usage

```bash
shredder <file> [passes]
```

Examples:

```bash
shredder secret.txt
shredder confidential.pdf 7
```

## License

GNU General Public License v3.0
