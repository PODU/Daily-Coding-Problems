// Connect 4 on a 7x6 grid. drop() places in lowest empty cell; after each move
// check 4-in-a-row in all 4 directions from the placed cell. Time O(1) per move.

const COLS: usize = 7;
const ROWS: usize = 6;

struct Connect4 {
    grid: [[u8; COLS]; ROWS],
    turn: u8,
}

impl Connect4 {
    fn new() -> Self {
        Connect4 { grid: [[b'.'; COLS]; ROWS], turn: b'R' }
    }

    fn drop(&mut self, col: usize) -> u8 {
        if col >= COLS || self.grid[ROWS - 1][col] != b'.' {
            return b'X';
        }
        let mut r = 0;
        while self.grid[r][col] != b'.' {
            r += 1;
        }
        self.grid[r][col] = self.turn;
        let who = self.turn;
        self.turn = if self.turn == b'R' { b'B' } else { b'R' };
        if self.wins(r as i32, col as i32, who) {
            who
        } else {
            b' '
        }
    }

    fn wins(&self, r: i32, c: i32, p: u8) -> bool {
        let dirs = [(0i32, 1i32), (1, 0), (1, 1), (1, -1)];
        for &(dr, dc) in &dirs {
            let mut cnt = 1;
            for s in [-1i32, 1] {
                for k in 1..4 {
                    let nr = r + dr * k * s;
                    let nc = c + dc * k * s;
                    if nr < 0 || nr >= ROWS as i32 || nc < 0 || nc >= COLS as i32
                        || self.grid[nr as usize][nc as usize] != p
                    {
                        break;
                    }
                    cnt += 1;
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
            let row: Vec<String> = self.grid[r].iter().map(|&c| (c as char).to_string()).collect();
            println!("{}", row.join(" "));
        }
    }
}

fn main() {
    let mut game = Connect4::new();
    let mut winner = b' ';
    for m in [0usize, 1, 0, 1, 0, 1, 0] {
        // Red wins vertically in column 0
        let res = game.drop(m);
        if res == b'R' || res == b'B' {
            winner = res;
            break;
        }
    }
    game.show();
    if winner != b' ' {
        println!("{} wins!", if winner == b'R' { "Red" } else { "Black" });
    } else {
        println!("No winner yet.");
    }
}
