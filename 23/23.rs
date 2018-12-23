// cargo-deps: text_io
#[macro_use]
extern crate text_io;

use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::io;
use std::io::prelude::*;
#[derive(Clone, Debug, Hash, PartialEq, Eq, Copy)]
struct Bot {
    x: i64,
    y: i64,
    z: i64,
    r: i64,
}

impl Bot {
    fn new(x: i64, y: i64, z: i64, r: i64) -> Bot {
        Bot { x, y, z, r }
    }

    fn dist(&self, other: &Bot) -> i64 {
        (self.x - other.x).abs() + (self.y - other.y).abs() + (self.z - other.z).abs()
    }

    fn div_by(&self, div: i64) -> Bot {
        Bot::new(self.x / div, self.y / div, self.z / div, self.r / div)
    }
    fn mul_by(&self, mul: i64) -> Bot {
        Bot::new(self.x * mul, self.y * mul, self.z * mul, self.r * mul)
    }

    fn in_range(&self, other: &Bot) -> bool {
        self.dist(other) <= self.r
    }
}

fn main() {
    let stdin = io::stdin();
    let values: Vec<Bot> = stdin
        .lock()
        .lines()
        .map(|x| {
            let (a, b, c, d): (i64, i64, i64, i64);
            let v = x.unwrap();
            scan!(v.bytes() => "pos=<{},{},{}>, r={}", a, b,c,d);
            Bot::new(a, b, c, d)
        })
        .collect();

    let biggest = values.iter().max_by_key(|x| x.r).unwrap();
    let in_range = values.iter().filter(|x| biggest.in_range(x)).count();
    println!("{:?} In range: {}", biggest, in_range);

    let mut div = 2 << 32;
    let origin = Bot::new(0, 0, 0, 0);
    let mut best = Bot::new(0, 0, 0, 0);
    loop {
        let lastbest = best.mul_by(2);
        let mut bestcount = 0;
        for x in (lastbest.x - 2)..=(lastbest.x + 2) {
            for y in (lastbest.y - 2)..=(lastbest.y + 2) {
                for z in (lastbest.z - 2)..=(lastbest.z + 2) {
                    let nbot = Bot::new(x, y, z, 0);
                    let count = values
                        .iter()
                        .filter(|x| x.div_by(div).in_range(&nbot))
                        .count() as i64;
                    if count > bestcount
                        || (count == bestcount && best.dist(&origin) > nbot.dist(&origin))
                    {
                        best = nbot;
                        bestcount = count;
                    }
                }
            }
        }
        if div == 1 {
            println!("{:?} {} {}", best, origin.dist(&best), bestcount);
            break;
        }
        div /= 2;

    }
}
