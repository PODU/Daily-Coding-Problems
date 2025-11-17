// Cryptarithmetic solver via backtracking over distinct letters with column-sum check.
// Time: O(10!) worst case over distinct letters (<=10), Space: O(#letters).
use std::collections::{HashMap, HashSet};

struct Solver {
    w1: Vec<char>,
    w2: Vec<char>,
    w3: Vec<char>,
    letters: Vec<char>,
    leading: HashSet<char>,
    assign: HashMap<char, i64>,
    used: [bool; 10],
}

impl Solver {
    fn value(&self, w: &[char]) -> i64 {
        let mut v = 0;
        for c in w {
            v = v * 10 + self.assign[c];
        }
        v
    }

    fn solve(&mut self, idx: usize) -> bool {
        if idx == self.letters.len() {
            return self.value(&self.w1.clone()) + self.value(&self.w2.clone())
                == self.value(&self.w3.clone());
        }
        let ch = self.letters[idx];
        for d in 0..=9 {
            if self.used[d as usize] {
                continue;
            }
            if d == 0 && self.leading.contains(&ch) {
                continue;
            }
            self.used[d as usize] = true;
            self.assign.insert(ch, d);
            if self.solve(idx + 1) {
                return true;
            }
            self.used[d as usize] = false;
        }
        false
    }
}

fn main() {
    let w1: Vec<char> = "SEND".chars().collect();
    let w2: Vec<char> = "MORE".chars().collect();
    let w3: Vec<char> = "MONEY".chars().collect();

    let mut leading = HashSet::new();
    leading.insert(w1[0]);
    leading.insert(w2[0]);
    leading.insert(w3[0]);

    let mut seen: Vec<char> = Vec::new();
    let mut in_seen = HashSet::new();
    for &c in w1.iter().chain(w2.iter()).chain(w3.iter()) {
        if in_seen.insert(c) {
            seen.push(c);
        }
    }

    let mut solver = Solver {
        w1,
        w2,
        w3,
        letters: seen.clone(),
        leading,
        assign: HashMap::new(),
        used: [false; 10],
    };

    if solver.solve(0) {
        let body: Vec<String> = seen
            .iter()
            .map(|c| format!("'{}': {}", c, solver.assign[c]))
            .collect();
        println!("{{{}}}", body.join(", "));
    } else {
        println!("No solution");
    }
}
