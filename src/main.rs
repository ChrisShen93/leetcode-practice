use std::io;

// mod fetch;

use crate::fetch;

fn main() {
    println!("Leetcode solutions in rust");

    let mut id_arg = String::new();
    io::stdin().read_line(&mut id_arg).expect("No problem id");

    let id = id_arg.parse::<u32>().expect("Failed transform to u32");

    fetch::get_problem(id);
}
