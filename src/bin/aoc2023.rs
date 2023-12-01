use clap::Parser;

use aoc2023::{log, tasks::solutions};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about=None)]
struct Args {
    #[clap(short, long)]
    pub day: i32,

    #[clap(short, long)]
    pub task: Option<i32>,

    #[clap(long, default_value_t = false)]
    pub debug: bool,
}

fn main() -> Result<(), anyhow::Error> {
    log::configure_log()?;

    let args = Args::parse();
    solutions::execute_task(&args.day, &args.task)?;

    Ok(())
}
