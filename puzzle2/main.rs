use std::io::{self, BufRead};

// read one single value from stdin, to facilitate initialization of a,b,c
fn read_next() -> i64 {
    let stdin = io::stdin();
    let x = stdin.lock().lines().next().unwrap().unwrap().parse::<i64>().unwrap();
    x
}

fn main() {
    let (mut a, mut b, mut c) = (read_next(), read_next(), read_next());
    let mut increased = 0;

    // iterate over lines in stdin
    for line in io::stdin().lock().lines() {
        let new = line.unwrap().trim().parse::<i64>().unwrap();
        if a < new {
            increased += 1;
        }
        a = b;
        b = c;
        c = new;
    }

    println!("{}", increased);
}
