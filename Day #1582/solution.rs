// Day 1582: Connect 4 on a 7x6 grid with win detection (H/V/diagonal).
// drop() places a disc; win() scans 4 directions from last move. Time: O(1) per move; Space: O(rows*cols).

const COLS: usize = 7;
const ROWS: usize = 6;

struct Connect4 {
    g: [[char; COLS]; ROWS],
}

impl Connect4 {
    fn new() -> Self {
        Connect4 { g: [['.'; COLS]; ROWS] }
    }
    fn drop(&mut self, col: usize, p: char) -> i32 {
        for r in (0..ROWS).rev() {
            if self.g[r][col] == '.' {
                self.g[r][col] = p;
                return r as i32;
            }
        }
        -1
    }
    fn win(&self, r: i32, c: i32) -> bool {
        let p = self.g[r as usize][c as usize];
        for &(dr, dc) in &[(0, 1), (1, 0), (1, 1), (1, -1)] {
            let mut cnt = 1;
            for s in [-1i32, 1] {
                let (mut rr, mut cc) = (r + s * dr, c + s * dc);
                while rr >= 0 && rr < ROWS as i32 && cc >= 0 && cc < COLS as i32
                    && self.g[rr as usize][cc as usize] == p
                {
                    cnt += 1;
                    rr += s * dr;
                    cc += s * dc;
                }
            }
            if cnt >= 4 {
                return true;
            }
        }
        false
    }
    fn show(&self) {
        for row in &self.g {
            let line: Vec<String> = row.iter().map(|ch| ch.to_string()).collect();
            println!("{}", line.join(" "));
        }
    }
}

fn main() {
    let mut game = Connect4::new();
    let moves = [(0, 'R'), (1, 'B'), (0, 'R'), (1, 'B'), (0, 'R'), (1, 'B'), (0, 'R')];
    let mut winner: Option<char> = None;
    for &(col, p) in &moves {
        let r = game.drop(col, p);
        if game.win(r, col as i32) {
            winner = Some(p);
            break;
        }
    }
    game.show();
    match winner {
        Some(w) => println!("Winner: {}", w), // R
        None => println!("Winner: none"),
    }
}
