# Setu - CLI Markdown Link Checker


### Motivation
After working on countless projects that all use markdown I find myself running into the same issue, are all my links valid?
A simple typo could redirect users to a dead website or a missing asset and these issues are often caught too late; in production.


## Installation:

Using cargo:

`cargo install setu-cli`

## Usage:

```
Usage: setu-cli [OPTIONS] [TARGET_PATH]

Arguments:
  [TARGET_PATH]  [default: .]

Options:
  -s, --strict
  -h, --help     Print help
  -V, --version  Print version
```

## Examples:
Use setu to scan the current directory in strict mode
`./setu-cli -s`

To scan a different directory type its path:
`./setu-cli ./docs`