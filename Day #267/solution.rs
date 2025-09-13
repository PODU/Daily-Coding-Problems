// Day 267: Is the black king in check? Locate K, then cast rook/queen orthogonal
// rays and bishop/queen diagonal rays (stopping at the first blocker), and test
// knight and pawn attacks. Time O(8x8) = O(1); space O(1).

fn in_check(board: &[&str]) -> bool {
    let g: Vec<Vec<u8>> = board.iter().map(|s| s.bytes().collect()).collect();
    let (mut kr, mut kc) = (-1i32, -1i32);
    for r in 0..8 {
        for c in 0..8 {
            if g[r][c] == b'K' {
                kr = r as i32;
                kc = c as i32;
            }
        }
    }
    if kr < 0 {
        return false;
    }
    let at = |r: i32, c: i32| -> u8 {
        if r < 0 || r >= 8 || c < 0 || c >= 8 {
            0
        } else {
            g[r as usize][c as usize]
        }
    };

    for &(dr, dc) in &[(1, 0), (-1, 0), (0, 1), (0, -1)] {
        let (mut r, mut c) = (kr + dr, kc + dc);
        while at(r, c) == b'.' {
            r += dr;
            c += dc;
        }
        let p = at(r, c);
        if p == b'R' || p == b'Q' {
            return true;
        }
    }
    for &(dr, dc) in &[(1, 1), (1, -1), (-1, 1), (-1, -1)] {
        let (mut r, mut c) = (kr + dr, kc + dc);
        while at(r, c) == b'.' {
            r += dr;
            c += dc;
        }
        let p = at(r, c);
        if p == b'B' || p == b'Q' {
            return true;
        }
    }
    let kn = [(1, 2), (1, -2), (-1, 2), (-1, -2), (2, 1), (2, -1), (-2, 1), (-2, -1)];
    for &(dr, dc) in &kn {
        if at(kr + dr, kc + dc) == b'N' {
            return true;
        }
    }
    if at(kr + 1, kc - 1) == b'P' || at(kr + 1, kc + 1) == b'P' {
        return true;
    }
    for dr in -1..=1 {
        for dc in -1..=1 {
            if (dr != 0 || dc != 0) && at(kr + dr, kc + dc) == b'k' {
                return true;
            }
        }
    }
    false
}

fn main() {
    let board = [
        "...K....",
        "........",
        ".B......",
        "......P.",
        ".......R",
        "..N.....",
        "........",
        ".....Q..",
    ];
    println!("{}", if in_check(&board) { "True" } else { "False" });
}
