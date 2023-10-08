```console
$ 03_06_global --help
A simple to use, efficient, and full-featured Command Line Argument Parser

Usage: 03_06_global [OPTIONS] <COMMAND>

Commands:
  greet
  help   Print this message or the help of the given subcommand(s)

Options:
      --name <NAME>
  -h, --help         Print help
  -V, --version      Print version

$ 03_06_global greet
Name: None

$ 03_06_global greet --name ferris
Name: Some("ferris")
```