# b91

A pure Rust implementation of a CLI utility for base91 encoding and decoding, similar to the standard `base64` command.

## Features

- **Base91 encoding/decoding** - More efficient than base64 for binary data
- **Pure Rust** - No external dependencies
- **File splitting** - Split output into multiple files with configurable size
- **Progress tracking** - Real-time progress bar with ETA and speed for file operations
- **Flexible output** - Wrap encoded output and customize file naming
- **Cross-platform** - Supports Linux, macOS, and Windows

## Installation

### From Release Binaries

Download the latest binary for your platform from the [releases page](https://github.com/ruckc/b91/releases):

- `b91-linux-amd64` - Linux x86_64
- `b91-linux-arm64` - Linux ARM64
- `b91-macos-amd64` - macOS Intel
- `b91-macos-arm64` - macOS Apple Silicon
- `b91-windows-amd64.exe` - Windows x86_64

### From Source

```bash
git clone https://github.com/ruckc/b91.git
cd b91
cargo build --release
```

The binary will be at `target/release/b91`.

## Usage

```
Usage: b91 [OPTION]... [FILE]

Encode or decode data using base91.

Options:
  -d, --decode           decode data
  -i, --ignore-garbage   when decoding, ignore non-base91 characters
  -w, --wrap=COLS        wrap encoded lines after COLS characters (default: no wrap)
  -s, --split=SIZE       split output into files of SIZE bytes each (see below)
  -n, --name=PREFIX      output file prefix (default: FILE.)
  --digits=N             number of digits in output file suffix (default: 3)
  --help                 display this help and exit

SIZE format:
  Integer and optional unit: K,M,G,T,P,E,Z,Y,R,Q (powers of 1024), KB,MB,... (powers of 1000),
  or binary prefixes: KiB=K, MiB=M, etc. Examples: 10K, 5MiB, 100MB

Output files: <prefix><number>.b91.txt, with <number> zero-padded to N digits.
If FILE is provided, input is read from FILE. Otherwise, input is read from stdin.
```

## Examples

### Basic Encoding/Decoding

```bash
# Encode from stdin
echo "hello world" | b91

# Decode from stdin
echo 'TPwJh>Io2Tv!^aB' | b91 -d

# Encode a file
b91 myfile.txt > encoded.txt

# Decode a file
b91 -d encoded.txt > decoded.txt
```

### Wrapped Output

```bash
# Wrap encoded output at 76 characters
echo "hello world" | b91 --wrap=76
```

### File Splitting

```bash
# Split encoded output into 1MB files
b91 --split=1M largefile.bin

# Custom output prefix and 4-digit numbering
b91 --split=500K -n chunk --digits=4 largefile.bin
# Creates: chunk0000.b91.txt, chunk0001.b91.txt, ...

# Split with progress bar (automatic when using file input)
b91 --split=10M largefile.bin
# Progress:  42.00% | ETA: 00:12 | Speed: 1.23 MB/s
```

### Ignore Garbage Characters

```bash
# Ignore non-base91 characters when decoding
b91 -d --ignore-garbage < encoded_with_whitespace.txt
```

## What is Base91?

Base91 is a binary-to-text encoding scheme that is more efficient than base64. While base64 uses 64 characters and has ~33% overhead, base91 uses 91 printable ASCII characters and has only ~23% overhead, making it more space-efficient for encoding binary data.

The base91 alphabet consists of:
```
ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789!#$%&()*+,./:;<=>?@[]^_`{|}~"
```

## Building

Requirements:
- Rust 1.70 or later

```bash
cargo build --release
```

## License

This project is licensed under the Apache License 2.0 - see the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please feel free to submit issues or pull requests.
