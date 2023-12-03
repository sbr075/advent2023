use crate::utils::{
    self,
    string::{find_first, reverse_string},
};

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
        _ => word.parse::<i32>()?,
    };

    Ok(number)
}

pub fn solution() -> Result<(), anyhow::Error> {
    let input = &utils::file::read_input()?;

    log::info!("Executing task 2");

    let re1 = "\\d|one|two|three|four|five|six|seven|eight|nine";
    let re2 = "\\d|enin|thgie|neves|xis|evif|ruof|eerht|owt|eno";

    let mut total = 0;
    for line in input {
        let reversed = reverse_string(line)?;
        let th = find_first(line, &re1)?;
        let nd = find_first(&reversed, &re2)?;
        let nd = reverse_string(&nd)?;

        let n1 = word_to_number(&th)?;
        let n2 = word_to_number(&nd)?;
        total += 10 * n1 + n2;
    }
    log::info!("solution: {}", total);

    Ok(())
}
