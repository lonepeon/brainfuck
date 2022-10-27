# Brainfuck

A little [Brainfuck interpreter and compiler](https://en.wikipedia.org/wiki/Brainfuck).

Some code examples copied from [Wikipedia](https://en.wikipedia.org/wiki/Brainfuck) and [Brainfuck.org](http://brainfuck.org/) are available in the examples folder. 

## Usage

```
Usage: brainfuck [OPTIONS] <SOURCE>

Arguments:
  <SOURCE>
          Path to the brainfuck program source code

Options:
  -m, --memory <MEMORY>
          Memory allocated when the brainfuck program starts. This is the initial memory but it can grow bigger if required
          
          [default: 4096]

  -x, --execution <EXECUTION>
          Define how the current program should behave
          
          [default: interpreter]
          [possible values: interpreter, compiler]

  -o, --output-dir <OUTPUT_FOLDER>
          Define where the compiled program should be generated. It's only used when using compiler execution mode
          
          [default: .]

  -h, --help
          Print help information (use `-h` for a summary)

  -V, --version
          Print version information
```

Usage examples:
- interpret a program `cargo run -- examples/tic-tac-toe.brainfuck`
- compile a program `cargo run -- -x compiler examples/tic-tac-toe.brainfuck`

## Tests

```
cargo test
```
