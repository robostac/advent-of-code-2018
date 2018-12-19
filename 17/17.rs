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
    Water,
    WaterRest,
    Clay,
    Ground,
}

fn flow_down(sx: i64, sy: i64, grid: &mut HashMap<(i64, i64), TileType>, maxy: i64) -> TileType {
    let k = (sx, sy);
    if sy > maxy {
        return TileType::Water;
    }
    if grid.contains_key(&k) {
        return grid[&k];
    }
    grid.insert(k, TileType::Water);
    let down = flow_down(sx, sy + 1, grid, maxy);
    if down == TileType::WaterRest || down == TileType::Clay {
        let mut at_rest = TileType::WaterRest;
        let mut fx = sx;
        let mut ex = sx;
        for v in &[-1, 1] {
            let mut cx = sx;
            loop {
                cx += v;
                let p = (cx, sy);
                if grid.contains_key(&p) {
                    break;
                }
                fx = min(fx, cx);
                ex = max(cx, ex);
                let down = flow_down(cx, sy + 1, grid, maxy);
                if (down == TileType::Water) {
                    at_rest = TileType::Water;
                    break;
                }
            }
        }
        for x in fx..=ex {
            grid.insert((x, sy), at_rest);
        }
    }
    grid[&k]
}

fn main() {
    let stdin = io::stdin();
    let mut maxy = 0;
    let mut miny = 1000000;
    let mut minx = 1000000;
    let mut maxx = 0;

    let values: Vec<(char, i64, i64, i64)> = stdin
        .lock()
        .lines()
        .map(|x| {
            let mut tempv = [0i64; 3];
            let start: char;
            let end: char;
            let v = x.unwrap();
            scan!(v.bytes() => "{}={}, {}={}..{}", start, tempv[0], end, tempv[1], tempv[2]);
            if (start == 'x') {
                maxy = max(maxy, tempv[2]);
                miny = min(miny, tempv[1]);
                minx = min(minx, tempv[0]);
                maxx = max(maxx, tempv[0]);
            } else {
                maxy = max(maxy, tempv[0]);
                miny = min(miny, tempv[0]);
                minx = min(minx, tempv[1]);
                maxx = max(maxx, tempv[2]);
            }
            (start, tempv[0], tempv[1], tempv[2])
        }).collect();
    let mut grid = HashMap::new();
    minx -= 1;
    maxx += 1;
    for v in values.iter() {
        if v.0 == 'x' {
            for y in v.2..=v.3 {
                grid.insert((v.1, y), TileType::Clay);
            }
        } else {
            for x in v.2..=v.3 {
                grid.insert((x, v.1), TileType::Clay);
            }
        }
    }
    flow_down(500, miny, &mut grid, maxy);
    /*
    for y in miny..=maxy {
        let mut s = String::new();
        for x in minx..=maxx {
            s.push(match grid.get(&(x, y)) {
                Some(TileType::Clay) => '#',
                Some(TileType::Water) => '|',
                Some(TileType::WaterRest) => '~',
                _ => '.',
            });
        }
        println!("{}", s);
    }*/
    let count = grid
        .iter()
        .filter(|x| (*x.1) == TileType::Water || (*x.1) == TileType::WaterRest)
        .count();
    println!("{}", count);
    let count = grid
        .iter()
        .filter(|x| (*x.1) == TileType::WaterRest)
        .count();
    println!("{}", count);
}
