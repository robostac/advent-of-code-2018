// cargo-deps: text_io
extern crate text_io;

use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::io;
use std::io::prelude::*;

type Point = (i64, i64);

fn add_point(first: &Point, other: &Point) -> Point {
    (first.0 + other.0, first.1 + other.1)
}

fn parse(
    s: &mut VecDeque<char>,
    grid: &mut HashMap<Point, char>,
    initial: HashSet<Point>,
) -> HashSet<Point> {
    let mut positions = HashSet::new();
    let mut current = initial.clone();
    loop {
        match s.pop_front().unwrap() {
            '^' => {}
            '(' => {
                current = parse(s, grid, current);
            }
            '$' | ')' => {
                positions.extend(current);
                break;
            }
            '|' => {
                positions.extend(current);
                current = initial.clone();
            }
            x => {
                let dir = match x {
                    'N' => (0, -1),
                    'S' => (0, 1),
                    'E' => (1, 0),
                    'W' => (-1, 0),
                    _ => panic!("unknown direction {}", x),
                };
                current = current
                    .iter()
                    .map(|cur_point| {
                        let door_pos = add_point(&cur_point, &dir);
                        let room_pos = add_point(&door_pos, &dir);
                        grid.insert(door_pos, '-');
                        grid.insert(room_pos, '.');
                        room_pos
                    }).collect();
            }
        }
    }
    positions
}

fn flood_fill_step(
    start: Vec<Point>,
    grid: &HashMap<Point, char>,
    visit: &mut HashSet<Point>,
) -> Vec<Point> {
    let mut next = Vec::new();
    for v in start {
        for m in &[(0, 1), (1, 0), (-1, 0), (0, -1)] {
            let pos = add_point(&v, &m);
            if grid.contains_key(&pos) == false {
                continue;
            }
            let pos = add_point(&pos, &m);
            if visit.contains(&pos) {
                continue;
            }
            visit.insert(pos.clone());
            next.push(pos);
        }
    }
    next
}

fn main() {
    let stdin = io::stdin();

    let mut text = String::new();
    stdin.read_line(&mut text);
    let mut cc: VecDeque<char> = text.chars().collect();

    let start = (0i64, 0i64);
    let mut initial = HashSet::new();
    initial.insert(start);

    let mut grid = HashMap::new();
    parse(&mut cc, &mut grid, initial);

    let mut cur = vec![start];
    let mut count = 0;
    let mut visit = HashSet::new();
    visit.insert(cur[0]);
    //print(&grid);
    for turn in 1.. {
        cur = flood_fill_step(cur, &grid, &mut visit);
        if cur.len() > 0 {
            if turn >= 1000 {
                count += cur.len();
            }
        } else {
            println!("{} {}", turn - 1, count);
            break;
        }
    }
}

fn print(grid: &HashMap<Point, char>) {
    let minx = grid.iter().map(|x| (x.0).0).min().unwrap();
    let maxx = grid.iter().map(|x| (x.0).0).max().unwrap();
    let miny = grid.iter().map(|x| (x.0).1).min().unwrap();
    let maxy = grid.iter().map(|x| (x.0).1).max().unwrap();
    for y in (miny - 1)..=(maxy + 1) {
        for x in (minx - 1)..=(maxx + 1) {
            print!(
                "{}",
                match grid.get(&(x, y)) {
                    Some(v) => *v,
                    None => '#',
                }
            );
        }
        println!();
    }
}
