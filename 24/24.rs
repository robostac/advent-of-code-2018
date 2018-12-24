// cargo-deps: text_io
#[macro_use]
extern crate text_io;

use std::cmp::Ordering;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::error::Error;
use std::io;
use std::io::prelude::*;
#[derive(Clone, Debug)]
struct Group {
    hp: i64,
    count: i64,
    damage: i64,
    attackType: String,
    weak: HashSet<String>,
    immune: HashSet<String>,
    id: i64,
    system: bool,
    init: i64,
}

impl Group {
    fn new() -> Group {
        Group {
            hp: 0,
            count: 0,
            damage: 0,
            attackType: "".to_owned(),
            weak: HashSet::new(),
            immune: HashSet::new(),
            id: 0,
            system: false,
            init: 0,
        }
    }

    fn eff_power(&self) -> i64 {
        self.count * self.damage
    }

    fn damage_val(&self, other: &Group) -> i64 {
        let v = self.eff_power();
        if other.immune.contains(&self.attackType) {
            0
        } else if other.weak.contains(&self.attackType) {
            v * 2
        } else {
            v
        }
    }
}

fn parser(text: &String) -> Result<Group, Box<Error>> {
    let mut g = Group::new();
    try_scan!(text.bytes() => "{} units each with {} hit points with an attack that does {} {} damage at initiative {}",  g.count, g.hp, g.damage, g.attackType, g.init );
    Ok(g)
}

fn parser_extra(text: &String) -> Result<Group, Box<Error>> {
    let mut g = Group::new();
    let mut extra = String::new();
    try_scan!(text.bytes() => "{} units each with {} hit points ({}) with an attack that does {} {} damage at initiative {}",  g.count, g.hp,extra, g.damage, g.attackType, g.init );
    let extra = extra.replace(",", "");
    for v in extra.split(';') {
        let mut x = v.trim().split(' ');
        let damtype = x.next().unwrap();
        for z in x.skip(1) {
            if (damtype == "weak") {
                g.weak.insert(z.to_string());
            } else {
                g.immune.insert(z.to_string());
            }
        }
    }
    Ok(g)
}

fn main() {
    let stdin = io::stdin();
    let values: Vec<String> = stdin.lock().lines().map(|x| x.unwrap()).collect();
    let mut units = Vec::new();
    let mut id = 0;
    let mut system = false;
    for v in values {
        if v == "Infection:" {
            system = true;
        } else if v == "Immune System:" {
            system = false;
        }
        match parser_extra(&v) {
            Ok(mut b) => {
                b.id = id;
                id += 1;
                b.system = system;
                units.push(b);
            }
            _ => {}
        }
        match parser(&v) {
            Ok(mut b) => {
                b.id = id;
                id += 1;
                b.system = system;
                units.push(b);
            }
            _ => {}
        }
    }

    for boost in 0.. {
        let mut active = units.clone();
        for x in active.iter_mut() {
            if x.system == false {
                x.damage += boost;
            }
        }
        let mut ids: Vec<usize> = (0..active.len()).collect();
        loop {
            let mut targets = HashMap::new();
            let mut used_as_target = HashSet::new();
            ids.sort_by_key(|x| -(active[*x].eff_power() * 100 + active[*x].init));
            for id in ids.iter() {
                let v = &active[*id];
                let mut poss_targets: Vec<usize> = ids
                    .iter().cloned()
                    .filter(|&x| {
                        (active[x].system != v.system)
                            && used_as_target.contains(&x) == false
                            && active[x].immune.contains(&v.attackType) == false
                    })
                    .collect();
                poss_targets.sort_by(|&x, &y| {
                    match v.damage_val(&active[x]).cmp(&v.damage_val(&active[y])) {
                        Ordering::Equal => {
                            match active[x].eff_power().cmp(&active[y].eff_power()) {
                                Ordering::Equal => active[x].init.cmp(&active[y].init),
                                other => other,
                            }
                        }
                        other => other,
                    }
                });
                match poss_targets.pop() {
                    Some(curtarget) => {
                        let bestdam = v.damage_val(&active[curtarget]);
                        if bestdam > 0 {
                            targets.insert(*id, curtarget);                            
                            used_as_target.insert(curtarget as usize);
                        }
                    }
                    _ => {}
                }
            }
            if targets.len() == 0 {
                break;
            }
            ids.sort_by_key(|&x| -active[x].init);
            for id in ids.iter().filter(|&x| targets.contains_key(x)) {
                if active[*id].count <= 0 {
                    continue;
                }
                let t = targets[id] as usize;
                let poss = active[*id].damage_val(&active[t]);
                let kill = poss / active[t].hp;
                active[t].count -= kill;
            }
            ids = ids.into_iter().filter(|&x| active[x].count > 0).collect();
            let immune = ids.iter().filter(|&&x| !active[x].system).count();
            let infect = ids.len() - immune;
            if infect == 0 || immune == 0 {
                let units: i64 = ids.iter().map(|x| active[*x].count).sum();
                if boost == 0 {
                    println!("{} {} {}", immune, infect, units);
                }
                if (immune > 0) {
                    println!("{} {} {} {}", immune, infect, units, boost);
                    return;
                }
                break;
            }
        }
    }
}
