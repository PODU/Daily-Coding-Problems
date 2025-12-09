// Day 720: Sudoku solver via backtracking with bitmasks for rows/cols/boxes,
// always filling the next empty cell. Time exponential worst-case but fast in practice.
struct Solver {
    grid: [[u8; 9]; 9],
    rows: [u16; 9],
    cols: [u16; 9],
    boxes: [u16; 9],
}

fn box_idx(r: usize, c: usize) -> usize {
    (r / 3) * 3 + c / 3
}

impl Solver {
    fn solve(&mut self, pos: usize) -> bool {
        if pos == 81 {
            return true;
        }
        let (r, c) = (pos / 9, pos % 9);
        if self.grid[r][c] != 0 {
            return self.solve(pos + 1);
        }
        let b = box_idx(r, c);
        for d in 1u8..=9 {
            let bit = 1u16 << d;
            if (self.rows[r] | self.cols[c] | self.boxes[b]) & bit != 0 {
                continue;
            }
            self.grid[r][c] = d;
            self.rows[r] |= bit;
            self.cols[c] |= bit;
            self.boxes[b] |= bit;
            if self.solve(pos + 1) {
                return true;
            }
            self.grid[r][c] = 0;
            self.rows[r] &= !bit;
            self.cols[c] &= !bit;
            self.boxes[b] &= !bit;
        }
        false
    }
}

fn main() {
    let puzzle = [
        "530070000", "600195000", "098000060",
        "800060003", "400803001", "700020006",
        "060000280", "000419005", "000080079",
    ];
    let mut s = Solver {
        grid: [[0; 9]; 9],
        rows: [0; 9],
        cols: [0; 9],
        boxes: [0; 9],
    };
    for (r, line) in puzzle.iter().enumerate() {
        for (c, ch) in line.bytes().enumerate() {
            let v = ch - b'0';
            s.grid[r][c] = v;
            if v != 0 {
                let bit = 1u16 << v;
                s.rows[r] |= bit;
                s.cols[c] |= bit;
                s.boxes[box_idx(r, c)] |= bit;
            }
        }
    }
    s.solve(0);
    for r in 0..9 {
        let line: String = (0..9).map(|c| (b'0' + s.grid[r][c]) as char).collect();
        println!("{}", line);
    }
}
