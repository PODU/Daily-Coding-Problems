// Conway's Game of Life on an infinite board: store live cells in a set; each step
// tally neighbor counts only around live cells, then apply the 4 rules.
// Time: O(L) per step (L live cells); Space: O(L). Printing bounds to the live region.
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
        for (cell, cnt) in counts {
            if cnt == 3 || (cnt == 2 && self.live.contains(&cell)) {
                next.insert(cell);
            }
        }
        self.live = next;
    }

    fn print(&self, step_no: usize) {
        println!("Step {}:", step_no);
        if self.live.is_empty() {
            println!("(empty)\n");
            return;
        }
        let min_r = self.live.iter().map(|&(r, _)| r).min().unwrap();
        let max_r = self.live.iter().map(|&(r, _)| r).max().unwrap();
        let min_c = self.live.iter().map(|&(_, c)| c).min().unwrap();
        let max_c = self.live.iter().map(|&(_, c)| c).max().unwrap();
        for r in min_r..=max_r {
            let mut row = String::new();
            for c in min_c..=max_c {
                row.push(if self.live.contains(&(r, c)) { '*' } else { '.' });
            }
            println!("{}", row);
        }
        println!();
    }
}

fn main() {
    let mut g = GameOfLife::new(&[(0, 0), (0, 1), (0, 2)]); // horizontal blinker
    let steps = 2;
    g.print(0);
    for s in 1..=steps {
        g.step();
        g.print(s);
    }
}
