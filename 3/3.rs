// cargo-deps: text_io
#[macro_use]
extern crate text_io;
use std::cmp::max;
use std::collections::HashSet;
use std::io;
use std::io::prelude::*;

struct Point {
    x: usize,
    y: usize,
}

struct Claim {
    id: usize,
    p: Point,
    s: Point,
}

fn main() {
    let stdin = io::stdin();
    let values: Vec<Claim> = stdin
        .lock()
        .lines()
        .map(|input| {
            let id: usize;
            let sx: usize;
            let sy: usize;
            let w: usize;
            let h: usize;
            let s = input.unwrap();
            scan!(s.bytes() => "#{} @ {},{}: {}x{}" , id, sx, sy, w, h);
            Claim {
                id,
                p: Point { x: sx, y: sy },
                s: Point { x: w, y: h },
            }
        }).collect();
    let mut cloth = [[0i32; 1000]; 1000];
    for v in values.iter() {
        for x in (v.p.x)..(v.p.x + v.s.x) {
            for y in (v.p.y)..(v.p.y + v.s.y) {
                cloth[x][y] += 1;
            }
        }
    }
    let overlapped: usize = cloth
        .iter()
        .map(|x| (*x).iter().filter(|y| **y > 1).count())
        .sum();
    println!("{:?}", overlapped);
    for v in values.iter() {
        let mut mv: i32 = 0;
        for x in (v.p.x)..(v.p.x + v.s.x) {
            for y in (v.p.y)..(v.p.y + v.s.y) {
                mv = max(mv, cloth[x][y])
            }
        }
        if mv == 1 {
            println!("{}", v.id);
        }
    }
}
