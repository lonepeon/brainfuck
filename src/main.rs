use brainfuck::{compiler, interpreter, optimizer, parser};
use clap::Parser;
use rand::distributions::DistString;

#[derive(clap::ValueEnum, Clone, Copy)]
enum ExecutionMode {
    Interpreter,
    Compiler,
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(
        short = 'm',
        long = "memory",
        default_value = "4096",
        long_help = "Memory allocated when the brainfuck program starts. This is the initial memory but it can grow bigger if required"
    )]
    memory: usize,
    #[arg(
        value_enum,
        short = 'x',
        long = "execution",
        default_value_t = ExecutionMode::Interpreter,
        long_help = "Define how the current program should behave"
    )]
    execution: ExecutionMode,
    #[arg(
        short = 'o',
        long = "output-dir",
        default_value = ".",
        long_help = "Define where the compiled program should be generated. It's only used when using compiler execution mode"
    )]
    output_folder: String,
    #[arg(long_help = "Path to the brainfuck program source code")]
    source: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();
    let stdin = std::io::stdin();
    let mut reader = std::io::BufReader::new(stdin);
    let source = std::fs::read_to_string(&args.source).expect("failed to read source file");
    let ast = optimizer::shrink_calls(&parser::parse(&source)?);

    match args.execution {
        ExecutionMode::Interpreter => {
            let mut out = std::io::stdout();
            interpreter::run(args.memory, &ast, &mut reader, &mut out)?;
        }
        ExecutionMode::Compiler => {
            let tmpdir_path = std::env::temp_dir();
            let tmpdir = tmpdir_path
                .to_str()
                .expect("failed to get the TMP directory");
            let random =
                rand::distributions::Alphanumeric.sample_string(&mut rand::thread_rng(), 5);
            let tmpfile = format!("{}/brainfuck-program-{}.rs", tmpdir, random);
            {
                let mut out = std::fs::File::create(&tmpfile)?;
                compiler::rust::generate(args.memory, &ast, &mut out)?;
            }
            let program_name = std::path::Path::new(&args.source)
                .file_stem()
                .and_then(|f| f.to_str())
                .unwrap_or("program.out");
            let outputfile = format!("{}/{}", args.output_folder, program_name);
            compiler::rust::compile(&tmpfile, &outputfile)?;
            std::fs::remove_file(tmpfile).expect("failed to remove temporary file");
        }
    }

    Ok(())
}
