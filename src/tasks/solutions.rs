use crate::tasks;

pub fn execute_task(day: &i32) -> Result<(), anyhow::Error> {
    use std::time::Instant;
    let now = Instant::now();

    match day {
        1 => {
            tasks::day1::task1::solution()?;
            tasks::day1::task2::solution()?;
        }
        2 => {
            tasks::day2::task1::solution()?;
            tasks::day2::task2::solution()?;
        }
        3 => {
            tasks::day3::task1::solution()?;
            tasks::day3::task2::solution()?;
        }
        _ => anyhow::bail!("invalid day"),
    };

    let elapsed = now.elapsed();
    log::info!("Elapsed: {:.2?}", elapsed);

    Ok(())
}
