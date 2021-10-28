mod matcher;

use matcher::find_matches;
use anyhow::{Context, Result};
use std::fs::File;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    #[structopt(long = "pattern")]
    pattern: String,

    #[structopt(long = "path")]
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    let args = Cli::from_args();

    let file = File::open(&args.path)
        .with_context(|| format!("Could not read file `{:#?}`", args.path))?;

    for number in find_matches(&file, &args.pattern) {
        println!("Matched line number {}", number + 1);
    }

    Ok(())
}
