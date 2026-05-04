// Chess check detection: locate black king, test pawn/knight attacks and ray-cast
// bishop/rook/queen lines blocked by pieces. Time: O(64); Space: O(1) extra.
fn in_check(b: &[&str]) -> bool {
    let g: Vec<Vec<char>> = b.iter().map(|s| s.chars().collect()).collect();
    let (mut kr, mut kc): (i32, i32) = (-1, -1);
    for r in 0..8 {
        for c in 0..8 {
            if g[r][c] == 'K' {
                kr = r as i32;
                kc = c as i32;
            }
        }
    }
    if kr < 0 {
        return false;
    }
    let at = |r: i32, c: i32| -> char {
        if r >= 0 && r < 8 && c >= 0 && c < 8 {
            g[r as usize][c as usize]
        } else {
            ' '
        }
    };

    // White pawns move up; a pawn at (kr+1, kc+-1) attacks the king.
    for dc in [-1, 1] {
        if at(kr + 1, kc + dc) == 'P' {
            return true;
        }
    }

    let km = [(1, 2), (1, -2), (-1, 2), (-1, -2), (2, 1), (2, -1), (-2, 1), (-2, -1)];
    for (dr, dc) in km {
        if at(kr + dr, kc + dc) == 'N' {
            return true;
        }
    }

    for (dr, dc) in [(1, 1), (1, -1), (-1, 1), (-1, -1)] {
        let (mut r, mut c) = (kr + dr, kc + dc);
        while r >= 0 && r < 8 && c >= 0 && c < 8 {
            let p = at(r, c);
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

    for (dr, dc) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
        let (mut r, mut c) = (kr + dr, kc + dc);
        while r >= 0 && r < 8 && c >= 0 && c < 8 {
            let p = at(r, c);
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
