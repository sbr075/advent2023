use crate::tasks;

pub fn execute_task(day: &i32, task: &Option<i32>) -> Result<(), anyhow::Error> {
    match day {
        1 => tasks::day1::solution(task),
        _ => anyhow::bail!("invalid day"),
    }
}
