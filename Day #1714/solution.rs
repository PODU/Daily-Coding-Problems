// Max words packed: boggle DFS to enumerate placements + backtracking over words. Exponential worst case, small dict.
use std::collections::BTreeSet;
use std::collections::HashSet;

const DR: [i32; 4] = [-1, 1, 0, 0];
const DC: [i32; 4] = [0, 0, -1, 1];

struct Solver {
    n: usize,
    board: Vec<Vec<u8>>,
    taken: Vec<Vec<bool>>,
    words: Vec<String>,
    used: HashSet<usize>,
    best: usize,
}

impl Solver {
    fn dfs(&self, r: i32, c: i32, w: &[u8], idx: usize, path: &mut BTreeSet<usize>, found: &mut HashSet<BTreeSet<usize>>) {
        if r < 0 || r >= self.n as i32 || c < 0 || c >= self.n as i32 {
            return;
        }
        let (ru, cu) = (r as usize, c as usize);
        let cell = ru * self.n + cu;
        if self.taken[ru][cu] || path.contains(&cell) || self.board[ru][cu] != w[idx] {
            return;
        }
        path.insert(cell);
        if idx == w.len() - 1 {
            found.insert(path.clone());
        } else {
            for d in 0..4 {
                self.dfs(r + DR[d], c + DC[d], w, idx + 1, path, found);
            }
        }
        path.remove(&cell);
    }

    fn find_placements(&self, w: &str) -> Vec<BTreeSet<usize>> {
        let wb = w.as_bytes();
        let mut found: HashSet<BTreeSet<usize>> = HashSet::new();
        for i in 0..self.n {
            for j in 0..self.n {
                let mut path = BTreeSet::new();
                self.dfs(i as i32, j as i32, wb, 0, &mut path, &mut found);
            }
        }
        found.into_iter().collect()
    }

    fn search(&mut self) {
        if self.used.len() > self.best {
            self.best = self.used.len();
        }
        let words = self.words.clone();
        for (wi, w) in words.iter().enumerate() {
            if self.used.contains(&wi) {
                continue;
            }
            for placement in self.find_placements(w) {
                for &cell in &placement {
                    self.taken[cell / self.n][cell % self.n] = true;
                }
                self.used.insert(wi);
                self.search();
                self.used.remove(&wi);
                for &cell in &placement {
                    self.taken[cell / self.n][cell % self.n] = false;
                }
            }
        }
    }
}

fn main() {
    let board: Vec<Vec<u8>> = vec![b"ean".to_vec(), b"tti".to_vec(), b"ara".to_vec()];
    let n = board.len();
    let mut solver = Solver {
        n,
        board,
        taken: vec![vec![false; n]; n],
        words: vec!["eat".into(), "rain".into(), "in".into(), "rat".into()],
        used: HashSet::new(),
        best: 0,
    };
    solver.search();
    println!("{}", solver.best);
}
