// cargo-deps: text_io
#[macro_use]
extern crate text_io;

use std::collections::HashMap;
use std::io;
use std::io::prelude::*;

fn main() {
    let stdin = io::stdin();
    let serial: usize;
    let mut s = String::new();
    stdin.read_line(&mut s);
    scan!(s.bytes() => "{}", serial);
    let exp = serial.to_string();
    println!("{}", exp);
    let mut cur: String = "37".to_owned();
    let mut elf = vec![0usize, 1];
    let mut index = 0;
    let mut found = false;
    while found == false {
        let curv1 = cur.as_bytes()[elf[0]] as usize - 48;
        let curv2 = cur.as_bytes()[elf[1]] as usize - 48;
        cur += &(curv1 + curv2).to_string();;
        if index > 0 {
            elf[0] = (elf[0] + curv1 + 1) % cur.len();
            elf[1] = (elf[1] + curv2 + 1) % cur.len();
        }
        index += 1;
        if index >= (serial + 10) {
            found = cur[(cur.len() - 10)..].contains(&exp);
        }
    }
    let s = &cur[serial..(serial + 10)];
    println!("{}", s);
    let pos = &cur.find(&exp);
    println!("f {}", pos.unwrap());
}
