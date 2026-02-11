use clap::{Parser, ValueEnum};
use parser::{Operation, ParseError, bin_format, csv_format, text_format};
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufReader, Read};

#[derive(Debug, Clone, ValueEnum)]
enum Format {
    Bin,
    Csv,
    Txt,
}

#[derive(Parser)]
#[command(name = "comparer")]
#[command(about = "Compare two YPBank operation files")]
struct Args {
    #[arg(long, help = "First file path")]
    file1: String,

    #[arg(long, help = "First file format")]
    format1: Format,

    #[arg(long, help = "Second file path")]
    file2: String,

    #[arg(long, help = "Second file format")]
    format2: Format,
}

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    // Read first file
    let file1 = File::open(&args.file1).map_err(|err| {
        eprintln!("Can't open file1 by specific path: {}", &args.file1);
        err
    })?;
    let reader1 = BufReader::new(file1);
    let operations1 = parse_file(reader1, &args.format1)?;

    // Read second file
    let file2 = File::open(&args.file2).map_err(|err| {
        eprintln!("Can't open file2 by specific path: {}", &args.file2);
        err
    })?;
    let reader2 = BufReader::new(file2);
    let operations2 = parse_file(reader2, &args.format2)?;

    // Compare
    if operations1.len() != operations2.len() {
        println!(
            "Files differ: {} has {} operations, {} has {} operations",
            args.file1,
            operations1.len(),
            args.file2,
            operations2.len()
        );
        return Ok(());
    }

    for operation in operations1.difference(&operations2) {
        println!("Operation with tx_id {} differs", operation.tx_id);
        return Ok(());
    }

    println!(
        "The operation records in '{}' and '{}' are identical.",
        args.file1, args.file2
    );

    Ok(())
}

fn parse_file<R: Read>(reader: R, format: &Format) -> Result<HashSet<Operation>, ParseError> {
    match format {
        Format::Bin => bin_format::parse_all(reader),
        Format::Csv => csv_format::parse_all(reader),
        Format::Txt => text_format::parse_all(reader),
    }
}
