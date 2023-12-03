use crate::utils::{self, string::split_string};

const WIDTH: usize = 11;
const HEIGHT: usize = 11;
const LENGTH: usize = WIDTH * HEIGHT;

fn is_number(char: &String) -> Result<bool, anyhow::Error> {
    let is_number = match char.parse::<i32>() {
        Ok(_) => true,
        Err(_) => false,
    };

    Ok(is_number)
}

fn is_symbol(char: &String) -> Result<bool, anyhow::Error> {
    let symbols = vec![
        "*".to_string(),
        "%".to_string(),
        "-".to_string(),
        "+".to_string(),
        "&".to_string(),
        "@".to_string(),
        "$".to_string(),
        "#".to_string(),
        "=".to_string(),
        "/".to_string(),
    ];
    Ok(symbols.contains(char))
}

fn create_bitmasks(chars: &Vec<String>) -> Result<([u8; LENGTH], [u8; LENGTH]), anyhow::Error> {
    let mut number_bitmap = [0u8; LENGTH];
    let mut symbol_bitmap = [0u8; LENGTH];

    for (i, char) in chars.iter().enumerate() {
        if is_number(char)? {
            number_bitmap[i] = 1;
        } else if is_symbol(char)? {
            let to_update = vec![
                i - 1,
                i + 1,
                i - WIDTH - 1,
                i - WIDTH,
                i - WIDTH + 1,
                i + WIDTH - 1,
                i + WIDTH,
                i + WIDTH + 1,
            ];

            for index in to_update {
                if index >= WIDTH * HEIGHT {
                    continue;
                }
                symbol_bitmap[index] = 1;
            }
        }
    }

    Ok((number_bitmap, symbol_bitmap))
}

fn sum_engine_parts(
    chars: &Vec<String>,
    number_bitmap: &[u8; LENGTH],
    symbol_bitmap: &[u8; LENGTH],
) -> Result<usize, anyhow::Error> {
    let engine_parts_bitmap: Vec<_> = number_bitmap
        .iter()
        .zip(symbol_bitmap)
        .map(|e| e.0 & e.1)
        .collect();

    let mut sum = 0;

    let mut is_engine_part = false;
    let mut buffer = Vec::new();
    for (i, bit) in number_bitmap.iter().enumerate() {
        if bit.eq(&1u8) {
            buffer.push(chars[i].parse::<usize>()?);

            if engine_parts_bitmap[i].eq(&1u8) {
                is_engine_part = true;
            }
        } else if bit.eq(&0u8) {
            if is_engine_part {
                let mut number = 0;
                for (j, n) in buffer.iter().rev().enumerate() {
                    number += 10usize.pow((j).try_into().unwrap()) * n;
                }
                sum += number;
                is_engine_part = false;
            }
            buffer.clear();
        }
    }

    Ok(sum)
}

pub fn solution() -> Result<(), anyhow::Error> {
    let input = utils::file::read_to_string()?.replace("\r\n", ".");

    log::info!("Executing task 1");

    let chars = split_string(&input, "")?;
    let (number_bitmap, symbol_bitmap) = create_bitmasks(&chars)?;
    let sum = sum_engine_parts(&chars, &number_bitmap, &symbol_bitmap)?;

    log::info!("solution {:?}", sum);
    Ok(())
}
