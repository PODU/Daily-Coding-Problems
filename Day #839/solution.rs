// Day 839: Max number of dictionary words packed on an NxN board.
// For each word collect all valid adjacent-path placements (DFS), then backtrack
// over words choosing a disjoint set to maximize the count.
// Time O(exponential worst-case) on small boards; placement search bounded by board size.

use std::collections::HashSet;

struct Board {
    mat: Vec<Vec<char>>,
    rows: usize,
    cols: usize,
}

impl Board {
    fn dfs(&self, word: &[char], r: usize, c: usize, idx: usize, used: u64,
           placements: &mut HashSet<u64>) {
        if self.mat[r][c] != word[idx] {
            return;
        }
        let used = used | (1u64 << (r * self.cols + c));
        if idx == word.len() - 1 {
            placements.insert(used);
            return;
        }
        let deltas: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];
        for (dr, dc) in deltas.iter() {
            let nr = r as i32 + dr;
            let nc = c as i32 + dc;
            if nr >= 0 && nr < self.rows as i32 && nc >= 0 && nc < self.cols as i32 {
                let (nr, nc) = (nr as usize, nc as usize);
                if used & (1u64 << (nr * self.cols + nc)) == 0 {
                    self.dfs(word, nr, nc, idx + 1, used, placements);
                }
            }
        }
    }

    fn find_placements(&self, word: &str) -> Vec<u64> {
        let chars: Vec<char> = word.chars().collect();
        let mut placements = HashSet::new();
        for i in 0..self.rows {
            for j in 0..self.cols {
                self.dfs(&chars, i, j, 0, 0, &mut placements);
            }
        }
        placements.into_iter().collect()
    }
}

fn backtrack(i: usize, occupied: u64, count: usize, wp: &[Vec<u64>], best: &mut usize) {
    if count > *best {
        *best = count;
    }
    if i == wp.len() {
        return;
    }
    backtrack(i + 1, occupied, count, wp, best); // skip
    for &tiles in &wp[i] {
        if occupied & tiles == 0 {
            backtrack(i + 1, occupied | tiles, count + 1, wp, best);
        }
    }
}

fn max_words(board: &Board, dict: &[&str]) -> usize {
    let mut wp: Vec<Vec<u64>> = Vec::new();
    for w in dict {
        let p = board.find_placements(w);
        if !p.is_empty() {
            wp.push(p);
        }
    }
    let mut best = 0;
    backtrack(0, 0, 0, &wp, &mut best);
    best
}

fn main() {
    let mat = vec![
        vec!['e', 'a', 'n'],
        vec!['t', 't', 'i'],
        vec!['a', 'r', 'a'],
    ];
    let board = Board {
        rows: mat.len(),
        cols: mat[0].len(),
        mat,
    };
    let dict = ["eat", "rain", "in", "rat"];
    println!("{}", max_words(&board, &dict));
}
