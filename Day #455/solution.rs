// Day 455: Conway's Game of Life on an infinite board.
// HashSet of live cells; tally neighbours each tick. Time O(live) per step.
use std::collections::{HashMap, HashSet};

type Cell = (i32, i32);

fn step(live: &HashSet<Cell>) -> HashSet<Cell> {
    let mut cnt: HashMap<Cell, i32> = HashMap::new();
    for &(r, c) in live {
        for dr in -1..=1 {
            for dc in -1..=1 {
                if dr != 0 || dc != 0 {
                    *cnt.entry((r + dr, c + dc)).or_insert(0) += 1;
                }
            }
        }
    }
    let mut next = HashSet::new();
    for (cell, n) in cnt {
        if n == 3 || (n == 2 && live.contains(&cell)) {
            next.insert(cell);
        }
    }
    next
}

fn print_board(live: &HashSet<Cell>) {
    if live.is_empty() {
        println!("(empty)");
        return;
    }
    let r0 = live.iter().map(|c| c.0).min().unwrap();
    let r1 = live.iter().map(|c| c.0).max().unwrap();
    let c0 = live.iter().map(|c| c.1).min().unwrap();
    let c1 = live.iter().map(|c| c.1).max().unwrap();
    for r in r0..=r1 {
        let mut line = String::new();
        for c in c0..=c1 {
            line.push(if live.contains(&(r, c)) { '*' } else { '.' });
        }
        println!("{}", line);
    }
}

fn main() {
    let mut live: HashSet<Cell> = [(1, 0), (1, 1), (1, 2)].into_iter().collect();
    let steps = 2;
    println!("Step 0:");
    print_board(&live);
    for s in 1..=steps {
        live = step(&live);
        println!("Step {}:", s);
        print_board(&live);
    }
}
