use std::{
    fs::File,
    io::{prelude::*, BufReader},
};

pub fn read_input() -> Result<Vec<String>, anyhow::Error> {
    let file = File::open("src/input.txt").expect("file does not exist");
    let buf = BufReader::new(file);
    let lines = buf
        .lines()
        .map(|l| l.expect("could not parse line"))
        .collect();

    Ok(lines)
}
