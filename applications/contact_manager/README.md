# contat_manager

## How to build

```bash
$ cargo check
$ cargo build
```

## How to run

```bash
$ cargo run -- --help
contact_manager 0.1.0
project: contact manager

USAGE:
    concat_manager.exe [FLAGS] [OPTIONS] <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
    -v               verbose

OPTIONS:
    -d <data-file>         [default: data.csv]

SUBCOMMANDS:
    add
    edit
    help      Prints this message or the help of the given subcommand(s)
    list
    remove
    search
```