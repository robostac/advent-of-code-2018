// cargo-deps: text_io
#[macro_use]
extern crate text_io;

use std::io;
use std::io::prelude::*;
#[derive(Clone, Debug)]
struct litem {
    next: usize,
    last: usize,
}
fn main() {
    let stdin = io::stdin();
    let mut s = String::new();
    stdin.read_line(&mut s);
    let mut max: usize;
    let playercount: usize;
    scan!(s.bytes() => "{} players; last marble is worth {} points", playercount, max );

    let mut marbles = Vec::new();
    marbles.resize(max * 100 + 1, litem { next: 0, last: 0 });
    let mut pos: usize = 0;
    let mut players = Vec::new();
    players.resize(playercount, 0usize);
    for cur in 1..=max * 100 {
        if cur % 23 == 0 {
            let p = cur % playercount;
            players[p] += cur;
            for i in 0..7 {
                pos = marbles[pos].last;
            }
            players[p] += pos;
            let last = marbles[pos].last;
            pos = marbles[pos].next;
            marbles[pos].last = last;
            marbles[last].next = pos;
        } else {
            let last = marbles[pos].next;
            let next = marbles[last].next;
            marbles[cur].next = next;
            marbles[cur].last = last;
            marbles[last].next = cur;
            marbles[next].last = cur;
            pos = cur;
        }
        if (cur == max) {
            println!("{:?}", players.iter().max());
        }
    }
    println!("{:?}", players.iter().max());
}
