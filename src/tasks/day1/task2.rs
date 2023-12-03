use crate::utils::{
    self,
    string::{find_first, reverse_string},
};

pub fn solution() -> Result<(), anyhow::Error> {
    let input = &utils::file::read_input()?;

    log::info!("Executing task 1");

    let re = "\\d";

    let mut total = 0;
    for line in input {
        let reversed = reverse_string(line)?;
        let th = find_first(line, re)?;
        let nd = find_first(&reversed, re)?;

        let n1 = th.parse::<i32>()?;
        let n2 = nd.parse::<i32>()?;
        total += 10 * n1 + n2;
    }
    log::info!("solution: {}", total);

    Ok(())
}
