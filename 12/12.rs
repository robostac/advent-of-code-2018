// cargo-deps: text_io
#[macro_use]
extern crate text_io;

use std::collections::HashMap;
use std::io;
use std::io::prelude::*;

fn nextGeneration(s: String, v: &HashMap<String, char>) -> String {
    let mut ns = String::new();
    let vs = "....".to_owned() + &s + "....";
    for i in 0..vs.len() - 4 {
        let cs = &vs[i..i + 5];
        if v.contains_key(cs) {
            ns.push(v[cs]);
        } else {
            ns.push('.');
        }
    }
    ns.to_string()
}

fn score(s: &String, offset: i64) -> i64 {
    let mut total = 0;
    for (p, v) in s.chars().enumerate() {
        if v == '#' {
            total += (p as i64) + offset;
        }
    }
    total
}

fn main() {
    let stdin = io::stdin();
    let mut s = String::new();
    stdin.read_line(&mut s);
    let initial: String;
    scan!(s.bytes() => "initial state: {}", initial);
    stdin.read_line(&mut s);

    let mut values: Vec<(String, char)> = stdin
        .lock()
        .lines()
        .map(|x| {
            let mut a = String::new();
            let mut b: char;
            let v = x.unwrap();
            scan!(v.bytes() => "{} => {}", a, b);
            (a, b)
        }).collect();

    let mut mm = HashMap::new();
    for v in values {
        mm.insert(v.0, v.1);
    }

    let mut cur = initial.clone();
    let mut offset: i64 = 0;
    for i in 0..20 {
        cur = nextGeneration(cur, &mm);
        offset -= 2;
    }
    println!("{}", score(&cur, offset));
    let mut diff = 0;
    let mut last = 0;
    let mut count = 0;
    let mut i = 20;
    let tgt = 50000000000;
    while (true) {
        i += 1;
        cur = nextGeneration(cur, &mm);
        offset -= 2;
        let s = score(&cur, offset);
        if (s - last) == diff {
            count += 1;
            if (count > 50) {
                last = s + ((tgt - i) * diff);
                break;
            }
        } else {
            diff = s - last;
            count = 0;
        }
        last = s;
    }
    println!("{}  ", last);
}
