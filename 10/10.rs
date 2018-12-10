// cargo-deps: text_io
#[macro_use]
extern crate text_io;

use std::io;
use std::io::prelude::*;

fn print(input: &Vec<(i64, i64, i64, i64)>) {
    let minx: i64 = input.iter().map(|&x| x.0).min().unwrap();
    let miny: i64 = input.iter().map(|&x| x.1).min().unwrap();
    let maxx: i64 = input.iter().map(|&x| x.0).max().unwrap();
    let maxy: i64 = input.iter().map(|&x| x.1).max().unwrap();
    for y in miny..=maxy {
        let mut s = String::new();
        for x in minx..=maxx {
            match (input.iter().find(|&p| p.0 == x && p.1 == y)) {
                None => s.push('.'),
                Some(v) => s.push('#'),
            }
        }
        println!("{}", s);
    }
}

fn main() {
    let stdin = io::stdin();
    let mut values: Vec<(i64, i64, i64, i64)> = stdin
        .lock()
        .lines()
        .map(|x| {
            let mut a = String::new();
            let mut b = String::new();
            let mut c = String::new();
            let mut d = String::new();
            let v = x.unwrap();
            scan!(v.bytes() => "position=<{},{}> velocity=<{},{}>",a, b, c ,d);
            (
                a.trim().parse::<i64>().unwrap(),
                b.trim().parse::<i64>().unwrap(),
                c.trim().parse::<i64>().unwrap(),
                d.trim().parse::<i64>().unwrap(),
            )
        }).collect();
    let mut lastx = 10000000;
    let mut c = 0;
    while true {
        let newvalues: Vec<(i64, i64, i64, i64)> = values
            .iter()
            .map(|&x| (x.0 + x.2, x.1 + x.3, x.2, x.3))
            .collect();
        let minx: i64 = newvalues.iter().map(|&x| x.0).min().unwrap();
        let maxx: i64 = newvalues.iter().map(|&x| x.0).max().unwrap();
        let dist = maxx - minx;
        if (dist >= lastx) {
            print(&values);
            println!("{}", c);
            break;
        }
        c += 1;
        values = newvalues;
        lastx = dist;
    }
}
