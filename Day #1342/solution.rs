// Day 1342: Pack the maximum number of dictionary words onto a letter grid (vertex-disjoint adjacency paths).
// Enumerate each word's placements (cell bitmasks) via DFS, then backtrack to select max disjoint set. Exponential worst case.
use std::collections::HashSet;

struct Packer {
    n: i32,
    m: i32,
    grid: Vec<Vec<char>>,
}

impl Packer {
    fn dfs(&self, w: &[char], idx: usize, r: i32, c: i32, mask: u64, out: &mut HashSet<u64>) {
        let mask = mask | (1u64 << (r * self.m + c));
        if idx == w.len() - 1 {
            out.insert(mask);
            return;
        }
        let dirs = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        for (dr, dc) in dirs.iter() {
            let (nr, nc) = (r + dr, c + dc);
            if nr < 0 || nr >= self.n || nc < 0 || nc >= self.m {
                continue;
            }
            if mask & (1u64 << (nr * self.m + nc)) != 0 {
                continue;
            }
            if self.grid[nr as usize][nc as usize] != w[idx + 1] {
                continue;
            }
            self.dfs(w, idx + 1, nr, nc, mask, out);
        }
    }
}

fn backtrack(placements: &[Vec<u64>], i: usize, used: u64, count: i32, best: &mut i32) {
    if count + (placements.len() - i) as i32 <= *best {
        return;
    }
    if i == placements.len() {
        if count > *best {
            *best = count;
        }
        return;
    }
    backtrack(placements, i + 1, used, count, best);
    for &m in &placements[i] {
        if m & used == 0 {
            backtrack(placements, i + 1, used | m, count + 1, best);
        }
    }
}

fn max_words(dict: &[&str], board: Vec<Vec<char>>) -> i32 {
    let packer = Packer { n: board.len() as i32, m: board[0].len() as i32, grid: board };
    let mut placements: Vec<Vec<u64>> = Vec::new();
    for w in dict {
        let chars: Vec<char> = w.chars().collect();
        let mut masks: HashSet<u64> = HashSet::new();
        for r in 0..packer.n {
            for c in 0..packer.m {
                if packer.grid[r as usize][c as usize] == chars[0] {
                    packer.dfs(&chars, 0, r, c, 0, &mut masks);
                }
            }
        }
        if !masks.is_empty() {
            placements.push(masks.into_iter().collect());
        }
    }
    let mut best = 0;
    backtrack(&placements, 0, 0, 0, &mut best);
    best
}

fn main() {
    let dict = ["eat", "rain", "in", "rat"];
    let board = vec![
        vec!['e', 'a', 'n'],
        vec!['t', 't', 'i'],
        vec!['a', 'r', 'a'],
    ];
    println!("{}", max_words(&dict, board));
}
