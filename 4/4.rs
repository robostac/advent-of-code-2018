// cargo-deps: text_io
#[macro_use]
extern crate text_io;
use std::cmp::max;
use std::collections::*;
use std::io;
use std::io::prelude::*;

struct Guard {
    id: usize,
    sleep: [usize; 60],
}
impl Guard {
    fn TotalSleep(&self) -> usize {
        self.sleep.iter().sum()
    }

    fn MaxSleep(&self) -> (usize, &usize) {
        self.sleep.iter().enumerate().max_by_key(|x| x.1).unwrap()
    }

    fn New(cur_guard: usize) -> Guard {
        Guard {
            id: cur_guard,
            sleep: [0usize; 60],
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let mut values: Vec<String> = stdin.lock().lines().map(|x| x.unwrap()).collect();
    values.sort();
    let mut guardlist = HashMap::new();
    let mut cur_guard: usize = 0;
    let mut start: usize = 0;
    for v in values.iter() {
        let hour: usize;
        let minute: usize;
        let action: String;
        let other: String;
        let ignore: String;
        scan!(v.bytes() => "[{} {}:{}] {} {}", ignore, hour, minute, action, other);
        match action.as_str() {
            "Guard" => {
                scan!(other.bytes() => "#{}", cur_guard);
            }
            "falls" => {
                start = minute;
            }
            "wakes" => {
                let mut g = guardlist.entry(cur_guard).or_insert(Guard::New(cur_guard));
                let end = minute;
                for i in start..end {
                    g.sleep[i] += 1
                }
            }
            _ => {}
        }
    }
    let max_guard = guardlist.iter().max_by_key(|x| x.1.TotalSleep()).unwrap().1;
    let minute = max_guard.MaxSleep().0;

    println!("{} {} {}", max_guard.id, minute, minute * max_guard.id);

    let max_single_minute_guard = guardlist.iter().max_by_key(|x| x.1.MaxSleep().1).unwrap().1;
    let minute = max_single_minute_guard.MaxSleep().0;

    println!(
        "{} {} {}",
        max_single_minute_guard.id,
        minute,
        minute * max_single_minute_guard.id
    );
}
