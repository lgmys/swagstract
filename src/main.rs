use anyhow::Result;
use clap::Parser;
use std::{fs, path::Path};

use crate::{operations::filter_operations_by_ids, spec::Spec};

mod components;
mod operations;
mod refs;
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
    let spec_file: String = fs::read_to_string(spec_path)?;
    let spec: Spec = serde_yaml::from_str(spec_file.as_str())?;

    let component_tree = components::ComponentTree::from_spec(&spec);

    let operations = operations::get_operations(&spec)?;

    let operations_filtered: Vec<operations::Operation> =
        filter_operations_by_ids(&operations, &args.operations.as_slice().to_vec());

    let components_matching_operations =
        component_tree.find_components_matching(&operations_filtered);

    let spec = spec::SpecBuilder::default()
        .set_components(components_matching_operations)
        .set_operations(operations_filtered)
        .build();

    let spec_as_string = serde_yaml::to_string(&spec)?;

    print!("{}", spec_as_string);

    Ok(())
}
