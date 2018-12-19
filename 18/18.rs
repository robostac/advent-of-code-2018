// cargo-deps: text_io
#[macro_use]
extern crate text_io;

use std::cmp::max;
use std::cmp::min;
use std::collections::HashMap;
use std::error::Error;
use std::io;
use std::io::prelude::*;
#[derive(Clone, Debug, Hash, PartialEq, Eq, Copy)]
enum TileType {
    Tree,
    Lumber,
    Open,
}

fn count(x: usize, y: usize, grid: &HashMap<(usize, usize), TileType>) -> (usize, usize, usize) {
    let mut open: usize = 0;
    let mut wood: usize = 0;
    let mut lumber: usize = 0;
    let points = [
        (x + 1, y),
        (x + 1, y + 1),
        (x, y + 1),
        (x - 1, y + 1),
        (x - 1, y),
        (x - 1, y - 1),
        (x, y - 1),
        (x + 1, y - 1),
    ];
    for p in points.iter() {
        match grid.get(p) {
            Some(TileType::Tree) => wood += 1,
            Some(TileType::Lumber) => lumber += 1,
            Some(TileType::Open) => open += 1,
            _ => (),
        }
    }
    (open, wood, lumber)
}

fn next(t: TileType, v: &(usize, usize, usize)) -> TileType {
    match t {
        TileType::Lumber => if v.1 >= 1 && v.2 >= 1 {
            TileType::Lumber
        } else {
            TileType::Open
        },
        TileType::Open => if v.1 >= 3 {
            TileType::Tree
        } else {
            TileType::Open
        },
        TileType::Tree => if v.2 >= 3 {
            TileType::Lumber
        } else {
            TileType::Tree
        },
    }
}

fn magic_minute(grid: &HashMap<(usize, usize), TileType>) -> HashMap<(usize, usize), TileType> {
    grid.iter()
        .map(|x| {
            (
                x.0.clone(),
                next(grid[&x.0], &count((x.0).0, (x.0).1, &grid)),
            )
        }).collect()
}

fn main() {
    let stdin = io::stdin();

    let values: Vec<(String)> = stdin.lock().lines().map(|x| x.unwrap()).collect();

    let mut grid = HashMap::new();
    for (y, s) in values.iter().enumerate() {
        for (x, v) in s.chars().enumerate() {
            grid.insert(
                (x, y),
                match v {
                    '#' => TileType::Lumber,
                    '|' => TileType::Tree,
                    '.' => TileType::Open,
                    _ => panic!("UNKNOWN TILE {}", v),
                },
            );
        }
    }
    let mut seen = HashMap::new();
    for turn in 0..1000000000 {
        grid = magic_minute(&grid);
        let mut s = String::new();
        for y in 0..50 {
            for x in 0..50 {
                s.push(match grid[&(x, y)] {
                    TileType::Lumber => '#',
                    TileType::Open => '.',
                    TileType::Tree => '|',
                })
            }
        }
        if seen.contains_key(&s) {
            let cycle = (turn - seen[&s]);
            let x = ((1000000000 - turn) / cycle) - 1;
            let turn = (turn + x * cycle) + 1;
            for _ in (turn)..1000000000 {
                grid = magic_minute(&grid);
            }
            break;
        }
        seen.insert(s, turn);
        if (turn == 10) {
            let wood = grid.iter().filter(|x| *x.1 == TileType::Tree).count();
            let lumber = grid.iter().filter(|x| *x.1 == TileType::Lumber).count();
            println!("{} {} {}", wood, lumber, wood * lumber);
        }
    }
    let wood = grid.iter().filter(|x| *x.1 == TileType::Tree).count();
    let lumber = grid.iter().filter(|x| *x.1 == TileType::Lumber).count();
    println!("{} {} {}", wood, lumber, wood * lumber);
}
