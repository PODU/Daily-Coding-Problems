// Day 1672: Determine if black king is in check on an 8x8 board.
// Scan knight jumps + 8 rays from king (first blocker decides). Time O(64), Space O(1).
fn in_check(b: &[&str]) -> bool {
    let grid: Vec<&[u8]> = b.iter().map(|s| s.as_bytes()).collect();
    let (mut kr, mut kc): (i32, i32) = (-1, -1);
    for r in 0..8 {
        for c in 0..8 {
            if grid[r][c] == b'K' {
                kr = r as i32;
                kc = c as i32;
            }
        }
    }
    let km = [(-2, -1), (-2, 1), (-1, -2), (-1, 2), (1, -2), (1, 2), (2, -1), (2, 1)];
    for (dr, dc) in km {
        let (r, c) = (kr + dr, kc + dc);
        if (0..8).contains(&r) && (0..8).contains(&c) && grid[r as usize][c as usize] == b'N' {
            return true;
        }
    }
    let dirs = [(1, 0), (-1, 0), (0, 1), (0, -1), (1, 1), (1, -1), (-1, 1), (-1, -1)];
    for (d, (dr, dc)) in dirs.iter().enumerate() {
        let diag = d >= 4;
        for step in 1..8 {
            let (r, c) = (kr + dr * step, kc + dc * step);
            if !((0..8).contains(&r) && (0..8).contains(&c)) {
                break;
            }
            let p = grid[r as usize][c as usize];
            if p == b'.' {
                continue;
            }
            if diag {
                if p == b'B' || p == b'Q' {
                    return true;
                }
                if step == 1 && p == b'K' {
                    return true;
                }
                if step == 1 && p == b'P' && *dr == 1 {
                    return true;
                }
            } else {
                if p == b'R' || p == b'Q' {
                    return true;
                }
                if step == 1 && p == b'K' {
                    return true;
                }
            }
            break;
        }
    }
    false
}

fn main() {
    let board = [
        "...K....", "........", ".B......", "......P.",
        ".......R", "..N.....", "........", ".....Q..",
    ];
    println!("{}", if in_check(&board) { "True" } else { "False" }); // True
}
