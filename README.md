# isousb

Tiny wrapper around dd and fdisk, written in Rust

## Dependencies

### Standard linux tools

fdisk, dd, sudo

### Rust libraries used (other than std)

- regex : <https://crates.io/crates/regex>
- users : <https://crates.io/crates/users>

## Installation

> cd target/release/
> cargo build --release
> sudo cp isousb /usr/bin/isousb