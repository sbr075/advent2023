use std::{
    fs::{self, File},
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

pub fn read_to_string() -> Result<String, anyhow::Error> {
    let string = fs::read_to_string("src/input.txt")?;
    Ok(string)
}
