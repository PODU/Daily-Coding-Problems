// Game of Life on infinite board: track live cells in a set, count neighbors via a
// map over live cells + neighbors each step. O(live*9) per step. Print cropped board.
use std::collections::{HashMap, HashSet};

fn step(live: &HashSet<(i32, i32)>) -> HashSet<(i32, i32)> {
    let mut counts: HashMap<(i32, i32), i32> = HashMap::new();
    for &(x, y) in live {
        for dx in -1..=1 {
            for dy in -1..=1 {
                if dx != 0 || dy != 0 {
                    *counts.entry((x + dx, y + dy)).or_insert(0) += 1;
                }
            }
        }
    }
    let mut next = HashSet::new();
    for (&cell, &n) in &counts {
        if n == 3 || (live.contains(&cell) && n == 2) {
            next.insert(cell);
        }
    }
    next
}

fn print_board(live: &HashSet<(i32, i32)>) {
    let minx = live.iter().map(|c| c.0).min().unwrap();
    let maxx = live.iter().map(|c| c.0).max().unwrap();
    let miny = live.iter().map(|c| c.1).min().unwrap();
    let maxy = live.iter().map(|c| c.1).max().unwrap();
    for y in miny..=maxy {
        let mut row = String::new();
        for x in minx..=maxx {
            row.push(if live.contains(&(x, y)) { '*' } else { '.' });
        }
        println!("{}", row);
    }
}

fn main() {
    let mut live: HashSet<(i32, i32)> = [(0, 1), (1, 1), (2, 1)].iter().cloned().collect();
    let steps = 2;
    for s in 0..=steps {
        println!("Step {}:", s);
        print_board(&live);
        if s < steps {
            live = step(&live);
        }
    }
}
