// cargo-deps: text_io
#[macro_use]
extern crate text_io;
use std::collections::*;
use std::io;
use std::io::prelude::*;

fn count(s: &str) -> (bool, bool) {
    let mut frequency: HashMap<char, i64> = HashMap::new();
    for x in s.chars() {
        *frequency.entry(x).or_insert(0) += 1;
    }
    let twos = frequency.iter().filter(|x| *x.1 == 2).count() > 0;
    let threes = frequency.iter().filter(|x| *x.1 == 3).count() > 0;
    (twos, threes)
}

fn diff(s1: &str, s2: &str) -> (String, bool) {
    let mut same = String::new();
    let mut diff = 0;
    let s2c: Vec<char> = s2.chars().collect();
    for (i, x) in s1.chars().enumerate() {
        if x == s2c[i] {
            same.push(x);
        } else {
            diff += 1;
        }
    }
    (same, diff == 1)
}

fn main() {
    let stdin = io::stdin();
    let values: Vec<String> = stdin.lock().lines().map(|x| x.unwrap()).collect();
    let res: Vec<(bool, bool)> = values.iter().map(|x| count(&x)).collect();
    let twos = res.iter().filter(|x| x.0).count();
    let threes = res.iter().filter(|x| x.1).count();
    println!("{}", twos * threes);

    for (i, x) in values.iter().enumerate() {
        for (j, y) in values[i + 1..].iter().enumerate() {
            let v = diff(x, y);
            if v.1 {
                println!("{}", v.0);
            }
        }
    }
}
