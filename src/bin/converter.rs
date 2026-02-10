use clap::{Parser, ValueEnum};
use parser::{Operation, ParseError, bin_format, csv_format, text_format};
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufReader, BufWriter, Read, Write};

#[derive(Debug, Clone, ValueEnum)]
enum Format {
    Binary,
    Csv,
    Text,
}

#[derive(Parser)]
#[command(name = "converter")]
#[command(about = "Convert YPBank operation files between formats")]
struct Args {
    #[arg(short, long, help = "Input file path")]
    input: String,

    #[arg(long, help = "Input format")]
    input_format: Format,

    #[arg(long, help = "Output format")]
    output_format: Format,
}

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    // Read operations
    let file = File::open(&args.input)?;
    let reader = BufReader::new(file);
    let operations = parse_input(reader, &args.input_format)?;

    // Write operations to stdout
    let stdout = io::stdout();
    let writer = BufWriter::new(stdout.lock());
    write_output(writer, &operations, &args.output_format)?;

    Ok(())
}

fn parse_input<R: Read>(reader: R, format: &Format) -> Result<HashSet<Operation>, ParseError> {
    match format {
        Format::Binary => bin_format::parse_all(reader),
        Format::Csv => csv_format::parse_all(reader),
        Format::Text => text_format::parse_all(reader),
    }
}

fn write_output<W: Write>(
    writer: W,
    operations: &HashSet<Operation>,
    format: &Format,
) -> Result<(), ParseError> {
    match format {
        Format::Binary => bin_format::write_all(writer, operations),
        Format::Csv => csv_format::write_all(writer, operations),
        Format::Text => text_format::write_all(writer, operations),
    }
}
