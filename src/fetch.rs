// use std::collections::HashMap;
extern crate reqwest;

const PROBLEMS_URL: &str = "https://leetcode.com/api/problems/algorithms/";

struct Problems {
    user_name: String,
}

pub async fn get_problems() -> Option<Problems> {
    let resp = reqwest::get(PROBLEMS_URL).await;
    println!("this is test {:#?}", resp);
    Ok(resp)
    // reqwest::get(PROBLEMS_URL)
}

pub fn get_problem(id: u32) -> () {
    get_problems().unwrap()
}

#[cfg(test)]
mod tests {}
