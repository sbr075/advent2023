use crate::utils::string;

fn word_to_number(word: &str) -> Result<&str, anyhow::Error> {
    let number = match word {
        "one" | "eno" => "1",
        "two" | "owt" => "2",
        "three" | "eerht" => "3",
        "four" | "ruof" => "4",
        "five" | "evif" => "5",
        "six" | "xis" => "6",
        "seven" | "neves" => "7",
        "eight" | "thgie" => "8",
        "nine" | "enin" => "9",
        _ => anyhow::bail!("how? {}", word),
    };

    Ok(number)
}

fn task1(input: &Vec<String>) -> Result<(), anyhow::Error> {
    log::info!("Executing task 1");
    
    Ok(())
}

fn task2(input: &Vec<String>) -> Result<(), anyhow::Error> {
    log::info!("Executing task 2");

    Ok(())
}

pub fn solution(task: &Option<i32>, input: &Vec<String>) -> Result<(), anyhow::Error> {
    log::info!("Finding solutions for day 1");

    // Not pretty
    match task {
        Some(task) => match task {
            1 => task1(input)?,
            2 => task2(input)?,
            _ => anyhow::bail!("invalid task number"),
        },
        None => {
            task1(input)?;
            task2(input)?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use rstest::*;

    #[rstest]
    fn test_task1() -> Result<(), anyhow::Error> {
        Ok(())
    }

    #[rstest]
    fn test_task2() -> Result<(), anyhow::Error> {
        Ok(())
    }
}
