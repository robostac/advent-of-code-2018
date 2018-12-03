// cargo-deps: text_io
#[macro_use]
extern crate text_io;
use std::collections::HashSet;
use std::io;
use std::io::prelude::*;
fn main() {
    let stdin = io::stdin();
    let values: Vec<i64> = stdin
        .lock()
        .lines()
        .map(|input| {
            let x: i64;
            let s = input.unwrap();
            scan!(s.bytes() => "{}", x);
            x
        }).collect();
    let total: i64 = values.iter().sum();
    println!("{}", total);

    //part 2
    let mut seen = HashSet::new();
    let mut current: i64 = 0;
    let mut offset: usize = 0;
    while true {
        current += values[offset % values.len()];
        if seen.contains(&current) {
            break;
        }
        seen.insert(current);
        offset = offset + 1;
    }
    println!("{}", current);
}
