use regex::Regex;

pub fn reverse_string(string: &String) -> Result<String, anyhow::Error> {
    let reversed = string.chars().rev().collect();
    Ok(reversed)
}

pub fn find_first(string: &String, regex: &str) -> Result<String, anyhow::Error> {
    let re = Regex::new(regex).unwrap();
    let result = re.find(string).unwrap().as_str().to_string();

    Ok(result)
}
