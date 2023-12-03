use std::collections::HashMap;

use crate::utils::{self, string::split_string};

fn iterate_pulls(pulls: &Vec<String>) -> Result<(), anyhow::Error> {
    let bag = HashMap::from([
        ("blue".to_string(), 14),
        ("red".to_string(), 12),
        ("green".to_string(), 13),
    ]);

    for pull in pulls {
        let cube = split_string(&pull, " ")?;
        let number = cube[0].parse::<i32>().unwrap();
        let color = cube[1].to_string();

        if bag.get(&color).unwrap() < &number {
            anyhow::bail!("not possible")
        }
    }
    Ok(())
}

fn iterate_sets(sets: &Vec<String>) -> Result<(), anyhow::Error> {
    for set in sets {
        let pulls = split_string(&set, ", ")?;
        iterate_pulls(&pulls)?;
    }
    Ok(())
}

pub fn solution() -> Result<(), anyhow::Error> {
    let input = &utils::file::read_input()?;

    log::info!("Executing task 1");
    let mut sum = 0;
    for (i, game) in input.iter().enumerate() {
        let sets = split_string(game, ": ")?;
        let sets = split_string(&sets[sets.len() - 1], "; ")?;

        if iterate_sets(&sets).is_ok() {
            sum += i + 1;
        }
    }
    log::info!("solution {}", sum);
    Ok(())
}
