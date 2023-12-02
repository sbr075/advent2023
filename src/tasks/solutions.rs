use crate::tasks;

pub fn execute_task(day: &i32, input: &Vec<String>) -> Result<(), anyhow::Error> {
    use std::time::Instant;
    let now = Instant::now();

    match day {
        1 => tasks::day1::solution(input),
        2 => tasks::day2::solution(input),
        _ => anyhow::bail!("invalid day"),
    }?;

    let elapsed = now.elapsed();
    log::info!("Elapsed: {:.2?}", elapsed);

    Ok(())
}
