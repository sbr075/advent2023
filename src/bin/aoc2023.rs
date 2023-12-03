use clap::Parser;

use aoc2023::{logger, tasks::solutions, utils};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about=None)]
struct Args {
    #[clap(short, long)]
    pub day: i32,
}

fn main() -> Result<(), anyhow::Error> {
    let args = Args::parse();
    logger::configure_log()?;

    solutions::execute_task(&args.day)?;
    Ok(())
}
