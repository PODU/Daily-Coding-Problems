// Sudoku solver: backtracking with row/col/box bitmasks; pick first empty cell.
// Time: exponential worst case, fast in practice. Space: O(1) extra (fixed 9x9).

struct Solver {
    rows: [u32; 9],
    cols: [u32; 9],
    boxes: [u32; 9],
    grid: [[u8; 9]; 9],
}

fn box_index(r: usize, c: usize) -> usize {
    (r / 3) * 3 + c / 3
}

impl Solver {
    fn solve(&mut self, pos: usize) -> bool {
        if pos == 81 {
            return true;
        }
        let (r, c) = (pos / 9, pos % 9);
        if self.grid[r][c] != b'0' && self.grid[r][c] != b'.' {
            return self.solve(pos + 1);
        }
        let b = box_index(r, c);
        for d in 1..=9u32 {
            let bit = 1u32 << d;
            if self.rows[r] & bit != 0 || self.cols[c] & bit != 0 || self.boxes[b] & bit != 0 {
                continue;
            }
            self.rows[r] |= bit;
            self.cols[c] |= bit;
            self.boxes[b] |= bit;
            self.grid[r][c] = b'0' + d as u8;
            if self.solve(pos + 1) {
                return true;
            }
            self.rows[r] &= !bit;
            self.cols[c] &= !bit;
            self.boxes[b] &= !bit;
            self.grid[r][c] = b'.';
        }
        false
    }
}

fn main() {
    let puzzle = [
        "53..7....",
        "6..195...",
        ".98....6.",
        "8...6...3",
        "4..8.3..1",
        "7...2...6",
        ".6....28.",
        "...419..5",
        "....8..79",
    ];
    let mut s = Solver {
        rows: [0; 9],
        cols: [0; 9],
        boxes: [0; 9],
        grid: [[b'.'; 9]; 9],
    };
    for r in 0..9 {
        let bytes = puzzle[r].as_bytes();
        for c in 0..9 {
            let ch = bytes[c];
            s.grid[r][c] = ch;
            if ch != b'.' && ch != b'0' {
                let bit = 1u32 << (ch - b'0') as u32;
                s.rows[r] |= bit;
                s.cols[c] |= bit;
                s.boxes[box_index(r, c)] |= bit;
            }
        }
    }
    s.solve(0);
    for r in 0..9 {
        println!("{}", String::from_utf8_lossy(&s.grid[r]));
    }
}
