use std::collections::HashMap;

use crate::utils::{self, string::split_string};

pub fn solution() -> Result<(), anyhow::Error> {
    let input = &utils::file::read_input()?;

    log::info!("Executing task 2");

    let mut sum = 0;
    for game in input {
        let mut max_cubes = HashMap::from([
            ("blue".to_string(), 0),
            ("red".to_string(), 0),
            ("green".to_string(), 0),
        ]);

        let sets = split_string(game, ": ")?;
        let sets = split_string(&sets[1], "; ")?;
        for set in sets {
            let cubes: Vec<_> = split_string(&set, ", ")
                .unwrap()
                .into_iter()
                .map(|e| split_string(&e, " ").unwrap())
                .collect();

            for cube in cubes {
                let number = cube[0].parse::<i32>().unwrap();
                let color = cube[1].to_string();

                if max_cubes.get(&color).unwrap() < &number {
                    max_cubes.insert(color, number);
                }
            }
        }

        let power: i32 = max_cubes.values().into_iter().product();
        sum += power;
    }

    log::info!("solution {}", sum);
    Ok(())
}
