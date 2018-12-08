// cargo-deps: text_io
#[macro_use]
extern crate text_io;
use std::io;
use std::io::prelude::*;

struct Node {
    children: Vec<Node>,
    meta: Vec<i64>,
}

impl Node {
    fn new() -> Node {
        return Node {
            children: Vec::new(),
            meta: Vec::new(),
        };
    }

    fn Parse(input: &mut Vec<i64>) -> Node {
        let children = input.pop().unwrap();
        let meta = input.pop().unwrap();
        let mut n = Node::new();
        for x in 0..children {
            n.children.push(Node::Parse(input));
        }
        for x in 0..meta {
            let v = input.pop();
            n.meta.push(v.unwrap());
        }
        n
    }

    fn MetaSum(&self) -> i64 {
        let childsum: i64 = self.children.iter().map(|x| x.MetaSum()).sum();
        let value: i64 = self.meta.iter().sum();
        value + childsum
    }

    fn MetaSumPart2(&self) -> i64 {
        if self.children.len() == 0 {
            self.MetaSum()
        } else {
            self.meta
                .iter()
                .filter_map(|&x| {
                    if x == 0 || (x - 1) >= self.children.len() as i64 {
                        None
                    } else {
                        Some(x - 1)
                    }
                }).map(|x| self.children[x as usize].MetaSumPart2())
                .sum()
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let mut s = String::new();
    stdin.read_line(&mut s);

    let mut values: Vec<i64> = s
        .split(" ")
        .map(|x| {
            let b: i64;
            scan!(x.bytes() => "{}", b);
            b
        }).collect();
    values.reverse();
    let tree = Node::Parse(&mut values);
    println!("{} {}", tree.MetaSum(), tree.MetaSumPart2());
}
