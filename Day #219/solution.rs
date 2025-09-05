// Day 219: Connect 4 (7 cols x 6 rows).
// Approach: drop into lowest empty cell; after each move scan 4 directions from it for 4-in-a-row.
// Win check O(1) per move; board O(W*H).
const W: usize = 7;
const H: usize = 6;

struct Connect4 {
    grid: [[char; W]; H],
    height: [usize; W],
}

impl Connect4 {
    fn new() -> Self {
        Connect4 { grid: [['.'; W]; H], height: [0; W] }
    }

    fn drop(&mut self, col: usize, player: char) -> i32 {
        if col >= W || self.height[col] >= H {
            return -1;
        }
        let r = self.height[col];
        self.height[col] += 1;
        self.grid[r][col] = player;
        r as i32
    }

    fn wins(&self, r: i32, c: i32, p: char) -> bool {
        let dirs = [(0i32, 1i32), (1, 0), (1, 1), (1, -1)];
        for (dr, dc) in dirs {
            let mut cnt = 1;
            for s in [-1i32, 1] {
                let (mut rr, mut cc) = (r + dr * s, c + dc * s);
                while rr >= 0 && rr < H as i32 && cc >= 0 && cc < W as i32
                    && self.grid[rr as usize][cc as usize] == p
                {
                    cnt += 1;
                    rr += dr * s;
                    cc += dc * s;
                }
            }
            if cnt >= 4 {
                return true;
            }
        }
        false
    }
}

fn main() {
    let mut g = Connect4::new();
    let moves = [(0, 'R'), (1, 'B'), (0, 'R'), (1, 'B'), (0, 'R'), (1, 'B'), (0, 'R')];
    let mut winner = ' ';
    for (col, player) in moves {
        let row = g.drop(col, player);
        if g.wins(row, col as i32, player) {
            winner = player;
            break;
        }
    }
    println!("Player {} wins!", winner); // R wins vertically in column 0
}
