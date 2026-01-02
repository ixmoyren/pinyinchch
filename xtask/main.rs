use crate::task::convert_to_rkyv;
use clap::{Parser, Subcommand};

mod task;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    action: Action,
}

#[derive(Subcommand, Debug)]
enum Action {
    #[command(about = "Convert the json file to a binary file in rkyv format.")]
    ConvertToRkyv,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    match cli.action {
        Action::ConvertToRkyv => convert_to_rkyv(),
    }?;
    Ok(())
}
