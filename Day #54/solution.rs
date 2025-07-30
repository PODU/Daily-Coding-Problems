// Day 54: Sudoku solver via backtracking with bitmask row/col/box constraints.
// Worst case exponential; bitmasks make pruning fast. Space: O(1).
struct Solver {
    rows: [u32; 9],
    cols: [u32; 9],
    boxes: [u32; 9],
}

impl Solver {
    fn solve(&mut self, g: &mut [[u8; 9]; 9], cell: usize) -> bool {
        if cell == 81 {
            return true;
        }
        let (r, c) = (cell / 9, cell % 9);
        let b = (r / 3) * 3 + c / 3;
        if g[r][c] != 0 {
            return self.solve(g, cell + 1);
        }
        for d in 1u8..=9 {
            let bit = 1u32 << d;
            if (self.rows[r] | self.cols[c] | self.boxes[b]) & bit != 0 {
                continue;
            }
            g[r][c] = d;
            self.rows[r] |= bit;
            self.cols[c] |= bit;
            self.boxes[b] |= bit;
            if self.solve(g, cell + 1) {
                return true;
            }
            g[r][c] = 0;
            self.rows[r] &= !bit;
            self.cols[c] &= !bit;
            self.boxes[b] &= !bit;
        }
        false
    }
}

fn main() {
    let mut g: [[u8; 9]; 9] = [
        [5, 3, 0, 0, 7, 0, 0, 0, 0],
        [6, 0, 0, 1, 9, 5, 0, 0, 0],
        [0, 9, 8, 0, 0, 0, 0, 6, 0],
        [8, 0, 0, 0, 6, 0, 0, 0, 3],
        [4, 0, 0, 8, 0, 3, 0, 0, 1],
        [7, 0, 0, 0, 2, 0, 0, 0, 6],
        [0, 6, 0, 0, 0, 0, 2, 8, 0],
        [0, 0, 0, 4, 1, 9, 0, 0, 5],
        [0, 0, 0, 0, 8, 0, 0, 7, 9],
    ];
    let mut s = Solver { rows: [0; 9], cols: [0; 9], boxes: [0; 9] };
    for r in 0..9 {
        for c in 0..9 {
            if g[r][c] != 0 {
                let bit = 1u32 << g[r][c];
                s.rows[r] |= bit;
                s.cols[c] |= bit;
                s.boxes[(r / 3) * 3 + c / 3] |= bit;
            }
        }
    }
    s.solve(&mut g, 0);
    for r in 0..9 {
        let line: String = g[r].iter().map(|d| d.to_string()).collect();
        println!("{}", line);
    }
}
