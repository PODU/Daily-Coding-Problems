// Conway's Game of Life on an infinite board using a set of live coordinates.
// Each step counts neighbours only around live cells.
// Time: O(L) per step, Space: O(L).
use std::collections::{HashMap, HashSet};

type Cell = (i32, i32);

fn step(live: &HashSet<Cell>) -> HashSet<Cell> {
    let mut cnt: HashMap<Cell, i32> = HashMap::new();
    for &(x, y) in live {
        for dx in -1..=1 {
            for dy in -1..=1 {
                if dx != 0 || dy != 0 {
                    *cnt.entry((x + dx, y + dy)).or_insert(0) += 1;
                }
            }
        }
    }
    let mut nb = HashSet::new();
    for (&cell, &c) in &cnt {
        if c == 3 || (c == 2 && live.contains(&cell)) {
            nb.insert(cell);
        }
    }
    nb
}

fn render(live: &HashSet<Cell>) {
    if live.is_empty() {
        println!("(empty)");
        return;
    }
    let minx = live.iter().map(|c| c.0).min().unwrap();
    let maxx = live.iter().map(|c| c.0).max().unwrap();
    let miny = live.iter().map(|c| c.1).min().unwrap();
    let maxy = live.iter().map(|c| c.1).max().unwrap();
    for x in minx..=maxx {
        let mut row = String::new();
        for y in miny..=maxy {
            row.push(if live.contains(&(x, y)) { '*' } else { '.' });
        }
        println!("{}", row);
    }
}

fn main() {
    let mut live: HashSet<Cell> = [(1, 0), (1, 1), (1, 2)].iter().cloned().collect();
    let steps = 2;
    for i in 0..=steps {
        println!("Step {}:", i);
        render(&live);
        println!();
        live = step(&live);
    }
}
