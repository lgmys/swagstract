# swagstract

## Description

This is a simple cli app that will extract selected operations (and related components) from a swagger file and output them to stdout.
Supports OpenAPI 3.*.

### Installation

Clone the repository and do `cargo install .` or do `cargo install swagstract` from [crates.io](http://crates.io).

```
```

### Usage

```
swagstract -f <swagger file> -o <operation id> [-o <operation id> ...]
```

### Features

- [x] Extracts single operation
- [x] Extracts multiple operations
- [x] Extracts related components and references