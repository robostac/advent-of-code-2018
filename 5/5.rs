// cargo-deps: text_io
#[macro_use]
extern crate text_io;
use std::cmp::max;
use std::collections::*;
use std::io;
use std::io::prelude::*;

fn ReduceReplace(s: &Vec<u8>, v: u8) -> Vec<u8> {
    let mut b: Vec<u8> = s
        .iter()
        .filter_map(|&x| {
            if x != v && x != (v ^ 0x20) {
                Some(x)
            } else {
                None
            }
        }).collect();
    let mut valid = true;
    while valid {
        valid = false;

        for i in 0..(b.len() - 1) {
            if b[i] == (b[i + 1] ^ 0x20) {
                b.remove(i);
                b.remove(i);
                valid = true;
                break;
            }
        }
    }
    return b;
}

fn main() {
    let stdin = io::stdin();
    let mut values: Vec<String> = stdin.lock().lines().map(|x| x.unwrap()).collect();
    let my_ascii_lower = "abcdefghijklmnopqrstuvwxyz";
    let v2 = ReduceReplace(&values[0].to_owned().into_bytes(), 0);
    println!("{}", v2.len());
    let best = my_ascii_lower
        .bytes()
        .map(|x| ReduceReplace(&v2, x).len())
        .min();

    println!("{}", best.unwrap());
}
