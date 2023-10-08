# rcat
Reverse cat

Concatenate and reverse the lines of a file or stdin.

```
Usage: rcat [NAME]

Arguments:
  [NAME]
          Name of the file to read. If not specified, reads from stdin

Options:
  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```

## Example: Reversing a file
```bash
$ rcat Cargo.toml

clap = { version = "4.4.6", features = ["derive"] }
rev_lines = "0.3.0"
[dependencies]

authors = ["Luiz Picanço <lpicanco@gmail.com>"]
edition = "2021"
version = "0.1.0"
name = "rcat"
[package]
```

## Example: Reversing from stdin
```bash
$ cat Cargo.toml | rcat

clap = { version = "4.4.6", features = ["derive"] }
rev_lines = "0.3.0"
[dependencies]

authors = ["Luiz Picanço <lpicanco@gmail.com>"]
edition = "2021"
version = "0.1.0"
name = "rcat"
[package]
```
