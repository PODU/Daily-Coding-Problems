// Day 1498: Conway's Game of Life on an infinite board using a HashSet of
// live (row,col) cells; per step tally neighbor counts over live cells.
// Time O(L) per step (L = live cells), Space O(L).
use std::collections::{HashMap, HashSet};

struct GameOfLife {
    live: HashSet<(i32, i32)>,
}

impl GameOfLife {
    fn new(cells: &[(i32, i32)]) -> Self {
        GameOfLife { live: cells.iter().cloned().collect() }
    }

    fn step(&mut self) {
        let mut counts: HashMap<(i32, i32), i32> = HashMap::new();
        for &(r, c) in &self.live {
            for dr in -1..=1 {
                for dc in -1..=1 {
                    if dr != 0 || dc != 0 {
                        *counts.entry((r + dr, c + dc)).or_insert(0) += 1;
                    }
                }
            }
        }
        let mut next = HashSet::new();
        for (cell, n) in counts {
            let alive = self.live.contains(&cell);
            if n == 3 || (alive && n == 2) {
                next.insert(cell);
            }
        }
        self.live = next;
    }

    fn render(&self) -> String {
        if self.live.is_empty() {
            return "(empty)".to_string();
        }
        let (mut min_r, mut max_r, mut min_c, mut max_c) =
            (i32::MAX, i32::MIN, i32::MAX, i32::MIN);
        for &(r, c) in &self.live {
            min_r = min_r.min(r);
            max_r = max_r.max(r);
            min_c = min_c.min(c);
            max_c = max_c.max(c);
        }
        let mut lines = Vec::new();
        for r in min_r..=max_r {
            let mut row = String::new();
            for c in min_c..=max_c {
                row.push(if self.live.contains(&(r, c)) { '*' } else { '.' });
            }
            lines.push(row);
        }
        lines.join("\n")
    }
}

fn main() {
    let mut game = GameOfLife::new(&[(0, 1), (1, 1), (2, 1)]);
    let steps = 2;
    println!("Step 0:");
    println!("{}", game.render());
    for s in 1..=steps {
        game.step();
        println!("Step {}:", s);
        println!("{}", game.render());
    }
}
