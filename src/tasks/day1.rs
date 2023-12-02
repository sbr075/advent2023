use crate::utils::string;

fn word_to_number(word: &str) -> Result<i32, anyhow::Error> {
    let number = match word {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => word.parse::<i32>()?
    };

    Ok(number)
}

fn task1(input: &Vec<String>) -> Result<(), anyhow::Error> {
    log::info!("Executing task 1");

    let re = "\\d";

    let mut total = 0;
    for line in input {
        let reversed = string::reverse_string(line)?;
        let th = string::find_first(line, re)?;
        let nd = string::find_first(&reversed, re)?;

        let n1 = th.parse::<i32>()?;
        let n2 = nd.parse::<i32>()?;
        total += 10 * n1 + n2;
    }
    log::info!("solution: {}", total);

    Ok(())
}

fn task2(input: &Vec<String>) -> Result<(), anyhow::Error> {
    log::info!("Executing task 2");

    let re1 = "\\d|one|two|three|four|five|six|seven|eight|nine";
    let re2 = "\\d|enin|thgie|neves|xis|evif|ruof|eerht|owt|eno";

    let mut total = 0;
    for line in input {
        let reversed = string::reverse_string(line)?;
        let th = string::find_first(line, &re1)?;
        let nd = string::find_first(&reversed, &re2)?;
        let nd = string::reverse_string(&nd)?;

        let n1 = word_to_number(&th)?;
        let n2 = word_to_number(&nd)?;
        total += 10 * n1 + n2;
    }
    log::info!("solution: {}", total);

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
