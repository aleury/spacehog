[![CI](https://github.com/aleury/spacehog/actions/workflows/ci.yml/badge.svg)](https://github.com/aleury/spacehog/actions/workflows/ci.yml)
[![Nightly](https://github.com/aleury/spacehog/actions/workflows/nightly.yml/badge.svg)](https://github.com/aleury/spacehog/actions/workflows/nightly.yml)
[![Audit](https://github.com/aleury/spacehog/actions/workflows/audit.yml/badge.svg)](https://github.com/aleury/spacehog/actions/workflows/audit.yml)

# Install with Cargo

```
$ cargo install spacehog
```

# Build from source

```
$ git clone git@github.com:aleury/spacehog.git

$ cd spacehog

$ cargo install --path .
```

# Usage

```
A simple utility for finding large files on your system.

Usage: spacehog [OPTIONS] [PATH]

Arguments:
  [PATH]  [default: .]

Options:
  -n <NUMBER>      [default: 5]
  -h, --help       Print help
  -V, --version    Print version
```

## Examples

```sh
# View the top 5 largest files under the current directory
$ spacehog

# View the top 10 largest files under the current directory
$ spacehog -n 10

# View the top 10 largest files under the given path
$ spacehog ./stuff -n 10
```
