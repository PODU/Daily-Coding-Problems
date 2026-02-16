// Connect 4 engine with O(1)-per-move win detection (scan 4 directions from last disc).
// Time per move O(1), Space O(R*C).
const R: usize = 6;
const C: usize = 7;

struct Connect4 {
    g: [[char; C]; R],
    cur: char,
    over: bool,
    winner: char,
}

impl Connect4 {
    fn new() -> Self {
        Connect4 { g: [['.'; C]; R], cur: 'R', over: false, winner: '.' }
    }
    fn drop(&mut self, col: usize) -> i32 {
        if col >= C || self.over {
            return -1;
        }
        for r in (0..R).rev() {
            if self.g[r][col] == '.' {
                self.g[r][col] = self.cur;
                return r as i32;
            }
        }
        -1
    }
    fn won(&self, r: usize, col: usize) -> bool {
        let p = self.g[r][col];
        for (dr, dc) in [(0i32, 1i32), (1, 0), (1, 1), (1, -1)] {
            let mut cnt = 1;
            for s in [-1i32, 1] {
                let (mut nr, mut nc) = (r as i32 + dr * s, col as i32 + dc * s);
                while nr >= 0 && nr < R as i32 && nc >= 0 && nc < C as i32
                    && self.g[nr as usize][nc as usize] == p
                {
                    cnt += 1;
                    nr += dr * s;
                    nc += dc * s;
                }
            }
            if cnt >= 4 {
                return true;
            }
        }
        false
    }
    fn full(&self) -> bool {
        (0..C).all(|j| self.g[0][j] != '.')
    }
    fn play(&mut self, col: usize) -> bool {
        let r = self.drop(col);
        if r < 0 {
            return false;
        }
        if self.won(r as usize, col) {
            self.over = true;
            self.winner = self.cur;
        } else if self.full() {
            self.over = true;
        } else {
            self.cur = if self.cur == 'R' { 'B' } else { 'R' };
        }
        true
    }
    fn show(&self) {
        for r in 0..R {
            let s: Vec<String> = (0..C).map(|c| self.g[r][c].to_string()).collect();
            println!("{}", s.join(" "));
        }
    }
}

fn main() {
    let mut game = Connect4::new();
    for m in [0, 1, 0, 1, 0, 1, 0] { // R wins vertically in column 0
        game.play(m);
    }
    game.show();
    if game.winner != '.' {
        println!("Player {} wins!", game.winner);
    } else {
        println!("Draw");
    }
}
