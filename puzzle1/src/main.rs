use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut cur;
    let mut prev = 100000;
    let mut increased = 0;

    // iterate over lines in stdin
    for line in stdin.lock().lines() {
        // unwrap line result, trim and parse as i64, unwrap result of parse
        cur = line.unwrap().trim().parse::<i64>().unwrap();
        if prev < cur {
            increased += 1;
        }
        prev = cur;
    }

    println!("{}", increased);
}
