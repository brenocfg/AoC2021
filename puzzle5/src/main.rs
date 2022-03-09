//use itertools::Itertools;

use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut bitvals: [i32; 12] = [0; 12];

    // iterate over lines in stdin
    for line in stdin.lock().lines() {
        let input = match line {
            Ok(v) => v,
            Err(_) => continue,
        };

        // 0 most common -> negative [i]
        // 1 most common -> positive [i]
        for (i, c) in input.chars().enumerate() {
            match c {
                '0' => bitvals[i] -= 1,
                '1' => bitvals[i] += 1,
                _ => (),
            }
        }
    }

    // construct binary string representation of gamma, then convert to integer
    let gamma_str: String = bitvals
        .iter()
        .map(|x| ((x > &0) as i32).to_string())
        .collect();
    let gamma = u64::from_str_radix(&gamma_str, 2).unwrap();

    // flip first 12 bits of gamma to compute epsilon
    let mask: u64 = 0b0000_1111_1111_1111;
    let epsilon = gamma ^ mask;

    // final value = gamma vs epsilon
    println!("{}", gamma * epsilon);
}
