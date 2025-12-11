// Day 733: Connect 4 on a 7x6 grid.
// Approach: 2D board + per-column heights; after each drop scan 4 directions from the
// placed disc for 4-in-a-row. Time: O(1) per move, Space: O(rows*cols).

const COLS: usize = 7;
const ROWS: usize = 6;

struct Connect4 {
    board: [[char; COLS]; ROWS],
    height: [usize; COLS],
}

impl Connect4 {
    fn new() -> Self {
        Connect4 { board: [['.'; COLS]; ROWS], height: [0; COLS] }
    }

    fn drop(&mut self, col: usize, color: char) -> bool {
        let r = self.height[col];
        self.height[col] += 1;
        self.board[r][col] = color;
        let dirs = [(0i32, 1i32), (1, 0), (1, 1), (1, -1)];
        for (dr, dc) in dirs {
            let mut cnt = 1;
            for s in [-1i32, 1] {
                for k in 1..4 {
                    let nr = r as i32 + dr * k * s;
                    let nc = col as i32 + dc * k * s;
                    if nr >= 0 && nr < ROWS as i32 && nc >= 0 && nc < COLS as i32
                        && self.board[nr as usize][nc as usize] == color {
                        cnt += 1;
                    } else {
                        break;
                    }
                }
            }
            if cnt >= 4 {
                return true;
            }
        }
        false
    }

    fn show(&self) {
        for r in (0..ROWS).rev() {
            let line: Vec<String> = self.board[r].iter().map(|c| c.to_string()).collect();
            println!("{}", line.join(" "));
        }
    }
}

fn main() {
    let mut game = Connect4::new();
    let moves = [0, 0, 1, 1, 2, 2, 3];
    let mut color = 'R';
    let mut won = false;
    for &m in &moves {
        won = game.drop(m, color);
        if won {
            break;
        }
        color = if color == 'R' { 'B' } else { 'R' };
    }
    game.show();
    println!("{}", if won {
        if color == 'R' { "Red wins!" } else { "Black wins!" }
    } else {
        "No winner yet"
    });
}
