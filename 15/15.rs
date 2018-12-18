// cargo-deps: text_io
#[macro_use]
extern crate text_io;

use std::cmp::max;
use std::collections::HashMap;
use std::collections::HashSet;
use std::io;
use std::io::prelude::*;

#[derive(Clone, Debug, Hash, PartialEq, Eq, Copy)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
struct Player {
    p: Point,
    t: TileType,
    h: i32,
    a: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, Copy)]
enum Direction {
    STAY,
    LEFT,
    RIGHT,
    UP,
    DOWN,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, Copy)]
struct Step {
    p: Point,
    d: Direction,
}

static PossibleMoves: [Step; 4] = [
    Step {
        p: Point { x: 0, y: -1 },
        d: Direction::UP,
    },
    Step {
        p: Point { x: -1, y: 0 },
        d: Direction::LEFT,
    },
    Step {
        p: Point { x: 1, y: 0 },
        d: Direction::RIGHT,
    },
    Step {
        p: Point { x: 0, y: 1 },
        d: Direction::DOWN,
    },
];

impl Step {
    fn adjacent(&self) -> Vec<Step> {
        PossibleMoves
            .iter()
            .map(|x| Step {
                p: Point::new(self.p.x + x.p.x, self.p.y + x.p.y),
                d: if self.d == Direction::STAY {
                    x.d
                } else {
                    self.d
                },
            }).collect()
    }
}

impl Player {
    fn new(p: Point, t: TileType) -> Player {
        Player { p, t, h: 200, a: 3 }
    }

    fn enemy(&self) -> TileType {
        if self.t == TileType::GOBLIN {
            TileType::ELF
        } else {
            TileType::GOBLIN
        }
    }
    fn find_target(&self, players: &Vec<Player>) -> Option<Point> {
        let s = Step {
            p: self.p,
            d: Direction::STAY,
        };
        let adj = s.adjacent();

        let mut targets: Vec<&Player> = players
            .iter()
            .filter(|ref x| {
                x.t != self.t && x.h > 0 && adj.iter().filter(|ref y| y.p == x.p).count() > 0
            }).collect();
        if targets.len() == 0 {
            None
        } else {
            targets.sort_by(|x, y| {
                if x.h != y.h {
                    x.h.cmp(&y.h)
                } else if x.p.y == y.p.y {
                    x.p.x.cmp(&y.p.x)
                } else {
                    x.p.y.cmp(&y.p.y)
                }
            });
            Some(targets[0].p)
        }
    }
    fn do_move(&mut self, grid: &mut HashMap<Point, TileType>) {
        let mut curp = vec![Step {
            p: self.p,
            d: Direction::STAY,
        }];
        let enemy = self.enemy();
        let mut visit = HashSet::new();
        while curp.len() > 0 {
            let mut next = Vec::new();
            let mut valid = Vec::new();
            for v in curp.iter() {
                if visit.contains(&v.p) {
                    continue;
                }
                visit.insert(v.p);
                for np in v.adjacent() {
                    if visit.contains(&np.p) {
                        continue;
                    }

                    if grid[&np.p] == enemy {
                        valid.push(v.clone())
                    } else if grid[&np.p] == TileType::EMPTY {
                        next.push(np)
                    }
                }
            }
            if valid.len() > 0 {
                valid.sort_by(|ref x, ref y| {
                    if x.p.y == y.p.y {
                        x.p.x.cmp(&y.p.x)
                    } else {
                        x.p.y.cmp(&y.p.y)
                    }
                });
                grid.insert(self.p, TileType::EMPTY);
                match valid[0].d {
                    Direction::LEFT => self.p.x -= 1,
                    Direction::RIGHT => self.p.x += 1,
                    Direction::UP => self.p.y -= 1,
                    Direction::DOWN => self.p.y += 1,
                    _ => {}
                }
                grid.insert(self.p, self.t);
                break;
            }
            curp = next;
        }
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, Copy)]
enum TileType {
    WALL,
    GOBLIN,
    ELF,
    EMPTY,
}

fn main() {
    let stdin = io::stdin();
    let values: Vec<(String)> = stdin.lock().lines().map(|x| x.unwrap()).collect();
    let mut maxx = 0;
    let mut maxy = 0;

    let mut players = Vec::new();
    let mut grid = HashMap::new();
    for (y, s) in values.iter().enumerate() {
        for (x, v) in s.chars().enumerate() {
            let p = Point::new(x as i32, y as i32);
            match v {
                '#' => {
                    grid.insert(p, TileType::WALL);
                }
                'G' => {
                    grid.insert(p, TileType::GOBLIN);
                    players.push(Player::new(p, TileType::GOBLIN));
                }
                'E' => {
                    grid.insert(p, TileType::ELF);
                    players.push(Player::new(p, TileType::ELF));
                }
                '.' => {
                    grid.insert(p, TileType::EMPTY);
                }
                _ => panic!("Unknown tile {}", v),
            };
            maxx = max(maxx, x);
            maxy = max(maxy, y);
        }
    }

    let mut minattack = 3;
    let mut maxattack = 100;
    let mut cur_attack = 3;
    let mut results = vec![0; 101];
    while minattack < (maxattack - 1) {
        let mut grid = grid.clone();
        let mut players = players.clone();

        for p in players.iter_mut().filter(|x| x.t == TileType::ELF) {
            p.a = cur_attack;
        }

        let mut rounds = 0;
        loop {
            players.sort_by(|x, y| {
                if x.p.y == y.p.y {
                    x.p.x.cmp(&y.p.x)
                } else {
                    x.p.y.cmp(&y.p.y)
                }
            });
            let mut all_dead = false;
            for i in 0..players.len() {
                if players[i].h <= 0 {
                    continue;
                }
                let targets = players
                    .iter()
                    .filter(|x| x.t != players[i].t && x.h > 0)
                    .count();

                if targets == 0 {
                    all_dead = true;
                    break;
                }
                players[i].do_move(&mut grid);
                let target = players[i].find_target(&players);

                match target {
                    Some(target) => for t in 0..players.len() {
                        if players[t].p == target {
                            players[t].h -= players[i].a;
                            if players[t].h <= 0 {
                                if cur_attack != 3 && players[t].t == TileType::ELF {
                                    all_dead = true;
                                }
                                grid.insert(players[t].p, TileType::EMPTY);
                            }
                        }
                    },
                    _ => {}
                }
            }
            if all_dead {
                break;
            }
            players = players
                .iter()
                .filter_map(|x| if x.h <= 0 { None } else { Some(x.clone()) })
                .collect();
            rounds += 1;
        }
        let remaining_hp: i32 = players.iter().filter(|x| x.h > 0).map(|x| x.h).sum();
        let remaining_gobs = players
            .iter()
            .filter(|x| x.h > 0 && x.t == TileType::GOBLIN)
            .count();
        results[cur_attack as usize] = remaining_hp * rounds;
        if remaining_gobs > 0 {
            minattack = cur_attack;
        } else {
            maxattack = cur_attack;
        }
        cur_attack = (minattack + maxattack) / 2;
    }
    println!("{}", results[3]);
    println!("{}", results[maxattack as usize]);
}
