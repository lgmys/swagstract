use std::path::Path;

use anyhow::Result;
use clap::Parser;

mod output;
mod spec;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    //// Multiple values, will be parsed as Vec
    #[arg(short)]
    operations: Vec<String>,

    #[arg(short)]
    file: String,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let spec_path = Path::new(args.file.as_str());

    let mut parsed_spec = spec::from_file(spec_path).unwrap();

    let desired_operations = parsed_spec
        .operations
        .iter()
        .filter(|operation| args.operations.contains(&operation.operation_id))
        .cloned()
        .collect();

    parsed_spec.operations = desired_operations;

    output::print(&parsed_spec);

    Ok(())
}
