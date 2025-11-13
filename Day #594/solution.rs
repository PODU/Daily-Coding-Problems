// Day 594: Max number of non-overlapping dictionary words packable on a letter board.
// Approach: for each word enumerate all adjacency placements (DFS) as cell bitmasks,
// then backtracking max set-packing to pick the most pairwise-disjoint placements.
// Time: O(words * board * 4^L) to enumerate + exponential packing on small boards.
use std::collections::HashSet;

struct Board {
    grid: Vec<Vec<u8>>,
    r: usize,
    c: usize,
}

impl Board {
    fn find(&self, w: &[u8], idx: usize, r: i32, c: i32, mask: u64, out: &mut HashSet<u64>) {
        if r < 0 || c < 0 || r >= self.r as i32 || c >= self.c as i32 {
            return;
        }
        let bit = 1u64 << (r as usize * self.c + c as usize);
        if mask & bit != 0 || self.grid[r as usize][c as usize] != w[idx] {
            return;
        }
        let mask = mask | bit;
        if idx == w.len() - 1 {
            out.insert(mask);
            return;
        }
        self.find(w, idx + 1, r + 1, c, mask, out);
        self.find(w, idx + 1, r - 1, c, mask, out);
        self.find(w, idx + 1, r, c + 1, mask, out);
        self.find(w, idx + 1, r, c - 1, mask, out);
    }
}

fn pack(placements: &[u64], i: usize, used: u64, cnt: usize, best: &mut usize) {
    if cnt > *best {
        *best = cnt;
    }
    for j in i..placements.len() {
        if placements[j] & used == 0 {
            pack(placements, j + 1, used | placements[j], cnt + 1, best);
        }
    }
}

fn max_words(board: &Board, dict: &[&str]) -> usize {
    let mut placements: Vec<u64> = Vec::new();
    for w in dict {
        let mut masks = HashSet::new();
        for r in 0..board.r {
            for c in 0..board.c {
                board.find(w.as_bytes(), 0, r as i32, c as i32, 0, &mut masks);
            }
        }
        placements.extend(masks);
    }
    let mut best = 0;
    pack(&placements, 0, 0, 0, &mut best);
    best
}

fn main() {
    let board = Board {
        grid: vec![vec![b'e', b'a', b'n'], vec![b't', b't', b'i'], vec![b'a', b'r', b'a']],
        r: 3,
        c: 3,
    };
    let dict = ["eat", "rain", "in", "rat"];
    println!("{}", max_words(&board, &dict)); // 3
}
