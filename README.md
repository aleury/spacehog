[![CI](https://github.com/aleury/dstats/actions/workflows/ci.yml/badge.svg)](https://github.com/aleury/dstats/actions/workflows/ci.yml)
[![Nightly](https://github.com/aleury/dstats/actions/workflows/nightly.yml/badge.svg)](https://github.com/aleury/dstats/actions/workflows/nightly.yml)
[![Audit](https://github.com/aleury/dstats/actions/workflows/audit.yml/badge.svg)](https://github.com/aleury/dstats/actions/workflows/audit.yml)

# Usage

```
$ dstats --help
Usage: dstats [OPTIONS] [PATH]

Arguments:
  [PATH]  [default: .]

Options:
  -n <NUMBER>      [default: 5]
  -h, --help       Print help
```

## Examples

```sh
# View the top 5 largest files under the current directory
$ dstats

# View the top 10 largest files under the current directory
$ dstats -n 10

# View the top 10 largest files under the given path
$ dstats ./stuff -n 10
```
