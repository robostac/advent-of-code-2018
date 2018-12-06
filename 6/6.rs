// cargo-deps: text_io
#[macro_use]
extern crate text_io;
use std::cmp::max;
use std::collections::*;
use std::io;
use std::io::prelude::*;

fn main() {
    let stdin = io::stdin();
    let mut values: Vec<(i64, i64)> = stdin
        .lock()
        .lines()
        .map(|x| {
            let a: i64;
            let b: i64;
            let v = x.unwrap();
            scan!(v.bytes() => "{}, {}",a, b);
            (a, b)
        }).collect();
    let minx = values.iter().min_by_key(|&x| x.0).unwrap().0;
    let miny = values.iter().min_by_key(|&x| x.1).unwrap().1;
    let maxx = values.iter().max_by_key(|&x| x.0).unwrap().0;
    let maxy = values.iter().max_by_key(|&x| x.1).unwrap().1;
    println!("{} {} {} {}", minx, miny, maxx, maxy);
    let mut cur: Vec<((i64, i64), i64)> = values
        .iter()
        .enumerate()
        .map(|x| (((x.1).0, (x.1).1), x.0 as i64))
        .collect();
    let mut counts: Vec<i64> = Vec::new();
    counts.resize(values.len(), 0);
    let moves: Vec<(i64, i64)> = vec![(1, 0), (0, 1), (-1, 0), (0, -1)];
    let mut visit = HashSet::new();
    let mut dist = 0;
    while cur.len() > 0 {
        dist += 1;
        let mut next = HashMap::new();
        for x in cur.iter() {
            visit.insert(x.0);
            if counts[x.1 as usize] >= -1 {
                counts[x.1 as usize] += 1;
            }
            for m in moves.iter() {
                let np = ((x.0).0 + m.0, (x.0).1 + m.1);
                if (np.0 < minx || np.0 > maxx || np.1 < miny || np.1 > maxy) {
                    counts[x.1 as usize] = -10;
                    continue;
                }

                let v = next.entry(np).or_insert(x.1);
                if *v != x.1 {
                    *v = -10;
                }
            }
        }
        cur = next
            .iter()
            .filter_map(|x| {
                if *x.1 >= 0 && visit.contains(x.0) == false {
                    Some(((*x.0), *x.1))
                } else {
                    visit.insert(*x.0);
                    None
                }
            }).collect();
    }
    println!("{:?}", counts.iter().enumerate().max_by_key(|x| x.1));

    let mut count = 0;
    for x in minx..=maxx {
        for y in miny..=maxy {
            let totaldist: i64 = values
                .iter()
                .map(|&c| (c.0 - x).abs() + (c.1 - y).abs())
                .sum();
            if totaldist < 10000 {
                count += 1;
            }
        }
    }
    println!("{}", count);
}
