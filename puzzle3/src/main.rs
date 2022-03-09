use itertools::Itertools;

use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut x = 0;
    let mut y = 0;

    // iterate over lines in stdin
    for line in stdin.lock().lines() {
        let input = match line {
            Ok(v) => v,
            Err(_) => continue,
        };

        let mut tokens = input.split(" ");
        let values = tokens.next_tuple();
        let (dir, val) = match values {
            Some(tup) => match tup {
                (a, b) => (a.to_string(), b.parse::<i64>().unwrap()),
            },
            None => continue,
        };

        match dir.as_str() {
            "forward" => x += val,
            "down" => y += val,
            "up" => y -= val,
            &_ => (),
        }
    }

    println!("{}", x * y);
}
