// cargo-deps: text_io
#[macro_use]
extern crate text_io;
use std::io;
use std::io::prelude::*;

fn main() {
    let stdin = io::stdin();
    let values: Vec<(u8, u8)> = stdin
        .lock()
        .lines()
        .map(|x| {
            let a: char;
            let b: char;
            let v = x.unwrap();
            scan!(v.bytes() => "Step {} must be finished before step {} can begin.",a, b);
            (a as u8, b as u8)
        }).collect();
    let mut letters: Vec<u8> = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".to_owned().into_bytes();

    let mut output = String::new();
    let mut values_copy = values.clone();
    while letters.len() > 0 {
        letters
            .iter()
            .find(|&&v| values_copy.iter().filter(|&x| x.1 == v).count() == 0)
            .cloned()
            .map(|v| {
                output.push(v as char);
                letters.retain(|&x| x != v);
                values_copy.retain(|&x| x.0 != v);
            });
    }

    println!("{}", output);
    let mut workers = vec![(0, 0), (0, 0), (0, 0), (0, 0), (0, 0)];
    let mut second = 0;
    let mut output2 = String::new();

    let mut values_copy = values.clone();
    let mut letters: Vec<u8> = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".to_owned().into_bytes();

    while output2.len() < 26 {
        for w in workers.iter_mut().filter(|ref x| x.0 <= 0) {
            letters
                .iter()
                .find(|&&v| values_copy.iter().filter(|&x| x.1 == v).count() == 0)
                .cloned()
                .map(|v| {
                    letters.retain(|&x| x != v);
                    w.1 = v;
                    w.0 = 61 + (v - 'A' as u8);
                });
        }
        second += 1;
        for w in workers.iter_mut().filter(|x| x.0 > 0) {
            w.0 -= 1;
            if w.0 == 0 {
                values_copy.retain(|&x| x.0 != w.1);
                output2.push(w.1 as char);
            }
        }
    }
    println!("{} {}", output2, second);
}
