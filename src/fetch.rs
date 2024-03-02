// use std::collections::HashMap;

const PROBLEMS_URL: &str = "https://leetcode.com/api/problems/algorithms/";

pub async fn get_problems() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get(PROBLEMS_URL).await;
    println!("this is test {:#?}", resp);
    Ok(())
}

#[cfg(test)]
mod tests {}
