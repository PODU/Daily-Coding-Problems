// King-in-check: locate K, test rook/queen lines, bishop/queen diagonals, knight, king, pawn.
// Row 0 is top; white pawns attack upward (toward smaller row). Time O(64), Space O(1).

fn in_board(r: i32, c: i32) -> bool {
    r >= 0 && r < 8 && c >= 0 && c < 8
}

fn is_check(b: &[&str]) -> bool {
    let grid: Vec<Vec<char>> = b.iter().map(|s| s.chars().collect()).collect();
    let (mut kr, mut kc) = (-1i32, -1i32);
    for r in 0..8 {
        for c in 0..8 {
            if grid[r][c] == 'K' {
                kr = r as i32;
                kc = c as i32;
            }
        }
    }
    let get = |r: i32, c: i32| grid[r as usize][c as usize];

    for &(dr, dc) in &[(1, 0), (-1, 0), (0, 1), (0, -1)] {
        let (mut r, mut c) = (kr + dr, kc + dc);
        while in_board(r, c) {
            let p = get(r, c);
            if p != '.' {
                if p == 'R' || p == 'Q' {
                    return true;
                }
                break;
            }
            r += dr;
            c += dc;
        }
    }
    for &(dr, dc) in &[(1, 1), (1, -1), (-1, 1), (-1, -1)] {
        let (mut r, mut c) = (kr + dr, kc + dc);
        while in_board(r, c) {
            let p = get(r, c);
            if p != '.' {
                if p == 'B' || p == 'Q' {
                    return true;
                }
                break;
            }
            r += dr;
            c += dc;
        }
    }
    for &(dr, dc) in &[(1, 2), (1, -2), (-1, 2), (-1, -2), (2, 1), (2, -1), (-2, 1), (-2, -1)] {
        let (r, c) = (kr + dr, kc + dc);
        if in_board(r, c) && get(r, c) == 'N' {
            return true;
        }
    }
    for dr in -1..=1 {
        for dc in -1..=1 {
            if dr == 0 && dc == 0 {
                continue;
            }
            let (r, c) = (kr + dr, kc + dc);
            if in_board(r, c) && get(r, c) == 'K' {
                return true;
            }
        }
    }
    for dc in &[-1, 1] {
        let (r, c) = (kr + 1, kc + dc);
        if in_board(r, c) && get(r, c) == 'P' {
            return true;
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
    println!("{}", if is_check(&board) { "True" } else { "False" }); // True
}
