// Max non-overlapping dictionary words on a board.
// (1) DFS enumerate every placement (bitmask of cells) per word. (2) Backtrack over
// placements choosing pairwise-disjoint sets to maximize count.
use std::collections::HashSet;

struct Ctx {
    r: usize,
    c: usize,
    board: Vec<Vec<u8>>,
}

fn dfs(ctx: &Ctx, w: &[u8], idx: usize, r: usize, c: usize, used: u32, masks: &mut HashSet<u32>) {
    if ctx.board[r][c] != w[idx] {
        return;
    }
    let cell = r * ctx.c + c;
    if used & (1 << cell) != 0 {
        return;
    }
    let used = used | (1 << cell);
    if idx == w.len() - 1 {
        masks.insert(used);
        return;
    }
    let dirs: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    for (dr, dc) in dirs.iter() {
        let nr = r as i32 + dr;
        let nc = c as i32 + dc;
        if nr >= 0 && nr < ctx.r as i32 && nc >= 0 && nc < ctx.c as i32 {
            dfs(ctx, w, idx + 1, nr as usize, nc as usize, used, masks);
        }
    }
}

fn backtrack(placements: &[(usize, u32)], start: usize, occupied: u32, used_words: u32, count: i32, best: &mut i32) {
    if count > *best {
        *best = count;
    }
    for i in start..placements.len() {
        let (w, m) = placements[i];
        if used_words & (1 << w) != 0 {
            continue;
        }
        if occupied & m != 0 {
            continue;
        }
        backtrack(placements, i + 1, occupied | m, used_words | (1 << w), count + 1, best);
    }
}

fn main() {
    let dict = ["eat", "rain", "in", "rat"];
    let board: Vec<Vec<u8>> = vec![
        b"ean".to_vec(),
        b"tti".to_vec(),
        b"ara".to_vec(),
    ];
    let ctx = Ctx { r: board.len(), c: board[0].len(), board };
    let mut placements: Vec<(usize, u32)> = Vec::new();
    for (wi, w) in dict.iter().enumerate() {
        let wb = w.as_bytes();
        let mut masks: HashSet<u32> = HashSet::new();
        for r in 0..ctx.r {
            for c in 0..ctx.c {
                dfs(&ctx, wb, 0, r, c, 0, &mut masks);
            }
        }
        for m in masks {
            placements.push((wi, m));
        }
    }
    let mut best = 0;
    backtrack(&placements, 0, 0, 0, 0, &mut best);
    println!("{}", best);
}
