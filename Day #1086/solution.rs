// Sudoku solver via backtracking with bitmask constraints (rows/cols/boxes).
// Worst-case exponential, fast in practice; Space O(1).
fn box_id(r: usize, c: usize) -> usize {
    (r / 3) * 3 + c / 3
}

fn solve(
    pos: usize,
    grid: &mut [[i32; 9]; 9],
    rows: &mut [i32; 9],
    cols: &mut [i32; 9],
    boxes: &mut [i32; 9],
) -> bool {
    if pos == 81 {
        return true;
    }
    let (r, c) = (pos / 9, pos % 9);
    if grid[r][c] != 0 {
        return solve(pos + 1, grid, rows, cols, boxes);
    }
    let b = box_id(r, c);
    for d in 1..=9 {
        let bit = 1 << d;
        if (rows[r] | cols[c] | boxes[b]) & bit == 0 {
            grid[r][c] = d;
            rows[r] |= bit;
            cols[c] |= bit;
            boxes[b] |= bit;
            if solve(pos + 1, grid, rows, cols, boxes) {
                return true;
            }
            grid[r][c] = 0;
            rows[r] &= !bit;
            cols[c] &= !bit;
            boxes[b] &= !bit;
        }
    }
    false
}

fn main() {
    let mut grid: [[i32; 9]; 9] = [
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
    let mut rows = [0i32; 9];
    let mut cols = [0i32; 9];
    let mut boxes = [0i32; 9];
    for r in 0..9 {
        for c in 0..9 {
            if grid[r][c] != 0 {
                let bit = 1 << grid[r][c];
                rows[r] |= bit;
                cols[c] |= bit;
                boxes[box_id(r, c)] |= bit;
            }
        }
    }
    solve(0, &mut grid, &mut rows, &mut cols, &mut boxes);
    for r in 0..9 {
        let s: String = grid[r].iter().map(|d| d.to_string()).collect();
        println!("{}", s);
    }
}
