// cargo-deps: text_io
#[macro_use]
extern crate text_io;

use std::collections::HashMap;
use std::io;
use std::io::prelude::*;

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}
#[derive(Clone, Debug)]
struct Cart {
    dir: Point,
    pos: Point,
    junctions: i64,
}

impl Cart {
    fn update(&mut self, tile: char) {
        match tile {
            '|' | '-' => {}
            '/' => {
                self.dir = Point {
                    x: -self.dir.y,
                    y: -self.dir.x,
                };
            }
            '\\' => {
                self.dir = Point {
                    x: self.dir.y,
                    y: self.dir.x,
                };
            }
            '+' => {
                let v = self.junctions % 3;
                if v == 0 {
                    self.dir = Point {
                        x: self.dir.y,
                        y: -self.dir.x,
                    }
                } else if v == 2 {
                    self.dir = Point {
                        x: -self.dir.y,
                        y: self.dir.x,
                    };
                }
                self.junctions += 1;
            }

            _ => {
                panic!("{} unknown tile", tile);
            }
        }
        self.pos.x += self.dir.x;
        self.pos.y += self.dir.y;
    }

    fn destroy(&mut self) {
        self.dir = Point { x: 0, y: 0 };
    }

    fn isDead(&self) -> bool {
        return self.dir.x == 0 && self.dir.y == 0;
    }
}

fn main() {
    let stdin = io::stdin();
    let mut values: Vec<(String)> = stdin.lock().lines().map(|x| x.unwrap()).collect();

    let mut grid = HashMap::new();
    let mut carts = Vec::new();
    for (y, s) in values.iter().enumerate() {
        for (x, v) in s.chars().enumerate() {
            let p = Point {
                x: x as i32,
                y: y as i32,
            };
            match v {
                ' ' => {}
                '>' => {
                    carts.push(Cart {
                        dir: Point { x: 1, y: 0 },
                        pos: p.clone(),
                        junctions: 0,
                    });
                    grid.insert(p, '-');
                }
                '<' => {
                    carts.push(Cart {
                        dir: Point { x: -1, y: 0 },
                        pos: p.clone(),
                        junctions: 0,
                    });
                    grid.insert(p, '-');
                }
                '^' => {
                    carts.push(Cart {
                        dir: Point { x: 0, y: -1 },
                        pos: p.clone(),
                        junctions: 0,
                    });
                    grid.insert(p, '|');
                }
                'v' => {
                    carts.push(Cart {
                        dir: Point { x: 0, y: 1 },
                        pos: p.clone(),
                        junctions: 0,
                    });
                    grid.insert(p, '|');
                }
                _ => {
                    grid.insert(p, v);
                }
            }
        }
    }
    let mut collide = false;
    while carts.len() > 1 {
        for i in 0..carts.len() {
            let tile = grid[&carts[i].pos];
            carts[i].update(tile);
            let np = carts[i].pos.clone();
            let collisions = carts
                .iter()
                .filter(|&x| x.isDead() == false && x.pos == np)
                .count();
            if collisions > 1 {
                if collide == false {
                    println!("{:?}", np);
                    collide = true;
                }
                for z in carts.iter_mut() {
                    if z.pos == np {
                        z.destroy();
                    }
                }
            }
        }
        carts = carts
            .iter()
            .filter_map(|x| if x.isDead() { None } else { Some(x.clone()) })
            .collect();
    }
    println!("{:?}", carts);
}
