# swagstract

## Description

This is a simple cli app that will extract selected operations from a swagger file and output them to stdout,
ready to be piped into brand new, filtered swagger spec.

### Usage

```
swagstract -f <swagger file> -o <operation id> [-o <operation id> ...]
```

### Features

- [x] Extracts single operation
- [x] Extracts multiple operations
- [x] Extracts related components and references