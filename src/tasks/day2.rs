use std::collections::HashMap;

use crate::utils::string::split_string;

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

fn task1(input: &Vec<String>) -> Result<(), anyhow::Error> {
    log::info!("Executing task 1");
    let mut sum = 0;
    for (i, game) in input.iter().enumerate() {
        let sets = split_string(game, ": ")?;
        let sets = split_string(&sets[sets.len() - 1], "; ")?;

        if iterate_sets(&sets).is_ok() {
            sum += i + 1;
        }
    }
    log::info!("{}", sum);
    Ok(())
}

fn task2(input: &Vec<String>) -> Result<(), anyhow::Error> {
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
    log::info!("{}", sum);
    Ok(())
}

pub fn solution(input: &Vec<String>) -> Result<(), anyhow::Error> {
    log::info!("Finding solutions for day 2");
    task1(input)?;
    task2(input)?;
    Ok(())
}
