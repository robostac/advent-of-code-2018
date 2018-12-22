// cargo-deps: text_io
#[macro_use]
extern crate text_io;

use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::io;
use std::io::prelude::*;

type Point = (i64, i64);
type Cave = (Point, i64);
type Step = (Cave, i64);

fn add_point(first: &Point, other: &Point) -> Point {
    (first.0 + other.0, first.1 + other.1)
}

fn parse(p: Point, initial: &mut HashMap<Point, i64>, depth: i64) -> i64 {
    if initial.contains_key(&p) {
        return initial[&p];
    }
    let res = if p.0 == 0 {
        p.1 * 48271
    } else if p.1 == 0 {
        p.0 * 16807
    } else {
        parse(add_point(&p, &(0, -1)), initial, depth)
            * parse(add_point(&p, &(-1, 0)), initial, depth)
    };
    let erosion = (res + depth) % 20183;
    initial.insert(p, erosion);
    erosion
}

fn is_valid(cell: i64, equip: i64) -> bool {
    if cell == 0 && equip == 0 {
        false
    } else if cell == 1 && (equip & TORCH) > 0 {
        false
    } else if cell == 2 && (equip & CLIMB) > 0 {
        false
    } else {
        true
    }
}
fn flood_fill_step(
    start: Vec<Cave>,
    grid: &HashMap<Point, i64>,
    visit: &mut HashMap<Cave, i64>,
    maxtime: i64,
) -> Vec<Cave> {
    let mut next = Vec::new();
    for v in start {
        for equip in 0..=3 {
            let start = visit[&v];
            if start > maxtime {
                continue;
            }
            if v.1 != equip {
                if (is_valid(grid[&v.0], equip) == false) {
                    continue;
                }
                let time = 7;
                let newCave = (v.0, equip);
                let v = *visit.get(&newCave).unwrap_or(&maxtime);
                if (start + time) <= v {
                    visit.insert(newCave, start + time);
                    next.push(newCave);
                }
            } else {
                for m in &[(0, 1), (1, 0), (-1, 0), (0, -1)] {
                    let pos = add_point(&v.0, &m);
                    let newCave = (pos, v.1);
                    let time = 1;
                    if grid.contains_key(&pos) == false {
                        continue;
                    }
                    if (is_valid(grid[&pos], v.1) == false) {
                        continue;
                    }
                    let v = *visit.get(&newCave).unwrap_or(&maxtime);
                    if (start + time) < v {
                        visit.insert(newCave, start + time);
                        next.push(newCave);
                    }
                }
            }
        }
    }
    next
}
const TORCH: i64 = 1;
const CLIMB: i64 = 2;
fn main() {
    let stdin = io::stdin();

    let mut text = String::new();
    let depth: i64;
    stdin.read_line(&mut text);
    scan!(text.bytes() => "depth: {}", depth);

    let mut text = String::new();
    let x: i64;
    let y: i64;
    stdin.read_line(&mut text);
    scan!(text.bytes() => "target: {},{}", x, y);

    let start = (0i64, 0i64);
    let mut initial = HashMap::new();
    let mut grid = HashMap::new();
    for x in 0..=(x * 10) {
        for y in 0..=(y * 10) {
            let p = (x, y);
            let v = parse(p, &mut initial, depth);
            grid.insert(p, v % 3);
        }
    }
    grid.insert((x, y), 0);
    // print(&grid);
    let risk: i64 = grid
        .iter()
        .filter(|&p| (p.0).0 <= x && (p.0).1 <= y)
        .map(|x| x.1)
        .sum();
    println!("{}", risk);

    let initcave = (start, 1);
    let mut visit = HashMap::new();
    let mut cur = Vec::new();
    visit.insert(initcave, 0);
    cur.push(initcave);
    let mut maxtime = x * y * 8;
    while cur.len() > 0 {
        cur = flood_fill_step(cur, &grid, &mut visit, maxtime);
        for equip in (1..=3).step_by(2) {
            maxtime = std::cmp::min(maxtime, *visit.get(&((x, y), equip)).unwrap_or(&maxtime));
        }
    }

    print!("{:?} ", (x, y));
    for equip in (1..=3).step_by(2) {
        if visit.contains_key(&((x, y), equip)) {
            print!("{:?} ", visit[&((x, y), equip)]);
        } else {
            print!(". ");
        }
    }
    println!();
}

fn print(grid: &HashMap<Point, i64>) {
    let minx = grid.iter().map(|x| (x.0).0).min().unwrap();
    let maxx = grid.iter().map(|x| (x.0).0).max().unwrap();
    let miny = grid.iter().map(|x| (x.0).1).min().unwrap();
    let maxy = grid.iter().map(|x| (x.0).1).max().unwrap();
    for y in (miny - 1)..=(maxy + 1) {
        for x in (minx - 1)..=(maxx + 1) {
            print!(
                "{}",
                match grid.get(&(x, y)) {
                    Some(v) => match v {
                        0 => '.',
                        1 => '=',
                        2 => '|',
                        _ => '*',
                    },
                    None => '#',
                }
            );
        }
        println!();
    }
}
