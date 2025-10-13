// Day 425: Chess check detection. Knight/pawn offsets + sliding rays (rook/bishop/queen)
// with blocking. O(1) board scan from the king's square.
fn main() {
    let board = vec![
        "...K....",
        "........",
        ".B......",
        "......P.",
        ".......R",
        "..N.....",
        "........",
        ".....Q..",
    ];
    let g: Vec<Vec<char>> = board.iter().map(|s| s.chars().collect()).collect();
    let n = 8i32;
    let (mut kr, mut kc) = (-1i32, -1i32);
    for i in 0..8 {
        for j in 0..8 {
            if g[i][j] == 'K' {
                kr = i as i32;
                kc = j as i32;
            }
        }
    }
    let inb = |r: i32, c: i32| r >= 0 && r < n && c >= 0 && c < n;
    let mut check = false;

    let knight = [(-2, -1), (-2, 1), (-1, -2), (-1, 2), (1, -2), (1, 2), (2, -1), (2, 1)];
    for (dr, dc) in knight.iter() {
        let (r, c) = (kr + dr, kc + dc);
        if inb(r, c) && g[r as usize][c as usize] == 'N' {
            check = true;
        }
    }

    for dc in [-1i32, 1] {
        let (r, c) = (kr + 1, kc + dc);
        if inb(r, c) && g[r as usize][c as usize] == 'P' {
            check = true;
        }
    }

    let orth = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    for (dr, dc) in orth.iter() {
        let (mut r, mut c) = (kr + dr, kc + dc);
        while inb(r, c) {
            let p = g[r as usize][c as usize];
            if p != '.' {
                if p == 'R' || p == 'Q' {
                    check = true;
                }
                break;
            }
            r += dr;
            c += dc;
        }
    }

    let diag = [(-1, -1), (-1, 1), (1, -1), (1, 1)];
    for (dr, dc) in diag.iter() {
        let (mut r, mut c) = (kr + dr, kc + dc);
        while inb(r, c) {
            let p = g[r as usize][c as usize];
            if p != '.' {
                if p == 'B' || p == 'Q' {
                    check = true;
                }
                break;
            }
            r += dr;
            c += dc;
        }
    }

    println!("{}", if check { "True" } else { "False" });
}
