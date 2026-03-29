// Day 1278: Sudoku solver via backtracking with row/col/box bitmasks.
// Time: exponential worst-case but fast with constraint pruning. Space O(1).
struct Solver {
    g: [[u8; 9]; 9],
    rowm: [u16; 9],
    colm: [u16; 9],
    boxm: [u16; 9],
}

fn bidx(r: usize, c: usize) -> usize {
    (r / 3) * 3 + c / 3
}

impl Solver {
    fn solve(&mut self, pos: usize) -> bool {
        if pos == 81 {
            return true;
        }
        let (r, c) = (pos / 9, pos % 9);
        if self.g[r][c] != 0 {
            return self.solve(pos + 1);
        }
        let b = bidx(r, c);
        let used = self.rowm[r] | self.colm[c] | self.boxm[b];
        for d in 1u8..=9 {
            let bit = 1u16 << d;
            if used & bit != 0 {
                continue;
            }
            self.g[r][c] = d;
            self.rowm[r] |= bit;
            self.colm[c] |= bit;
            self.boxm[b] |= bit;
            if self.solve(pos + 1) {
                return true;
            }
            self.g[r][c] = 0;
            self.rowm[r] &= !bit;
            self.colm[c] &= !bit;
            self.boxm[b] &= !bit;
        }
        false
    }
}

fn main() {
    let puzzle = [
        "53..7....", "6..195...", ".98....6.",
        "8...6...3", "4..8.3..1", "7...2...6",
        ".6....28.", "...419..5", "....8..79",
    ];
    let mut s = Solver { g: [[0; 9]; 9], rowm: [0; 9], colm: [0; 9], boxm: [0; 9] };
    for (r, row) in puzzle.iter().enumerate() {
        for (c, ch) in row.chars().enumerate() {
            let v = if ch == '.' { 0 } else { ch as u8 - b'0' };
            s.g[r][c] = v;
            if v != 0 {
                let bit = 1u16 << v;
                s.rowm[r] |= bit;
                s.colm[c] |= bit;
                s.boxm[bidx(r, c)] |= bit;
            }
        }
    }
    s.solve(0);
    for r in 0..9 {
        let line: String = (0..9).map(|c| (b'0' + s.g[r][c]) as char).collect();
        println!("{}", line);
    }
}
