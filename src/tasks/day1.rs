fn task1() -> Result<(), anyhow::Error> {
    log::info!("Executing task 1");
    Ok(())
}

fn task2() -> Result<(), anyhow::Error> {
    log::info!("Executing task 2");
    Ok(())
}

pub fn solution(task: &Option<i32>) -> Result<(), anyhow::Error> {
    log::info!("Finding solutions for day 1");

    // Not pretty
    match task {
        Some(task) => match task {
            1 => task1()?,
            2 => task2()?,
            _ => anyhow::bail!("invalid task number"),
        },
        None => {
            task1()?;
            task2()?;
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
