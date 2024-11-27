# uuidgen
A command line tool to generate UUIDs and copy them to clipboard.

## Features

- Generates random UUIDs (v4)
- Automatically copies generated UUIDs to clipboard
- Can generate multiple UUIDs at once

## Usage

Generate a single UUID:
```
uuidgen
```

Generate a multiple UUIDs:
```
uuidgen 10
```

## Installation
1. Setup Rust
Follow the installation steps [here](https://www.rust-lang.org/tools/install).

2. Install with cargo
```shell
# root
cargo install --path .
```