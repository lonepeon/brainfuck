use brainfuck::{optimizer, parser, runner};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = std::env::args();
    let sourcefile = args
        .nth(1)
        .expect("source file must be passed as parameter");

    let stdin = std::io::stdin();
    let mut reader = std::io::BufReader::new(stdin);
    let mut stdout = std::io::stdout();
    let source = std::fs::read_to_string(sourcefile).expect("failed to read source file");

    let ast = optimizer::shrink_calls(&parser::parse(&source)?);
    runner::run(4096, &ast, &mut reader, &mut stdout)?;

    Ok(())
}
