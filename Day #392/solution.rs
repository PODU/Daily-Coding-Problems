// Island perimeter: +4 per land cell, -2 per adjacent right/down land pair. O(R*C) time, O(1) space.
fn island_perimeter(g: &Vec<Vec<i32>>) -> i32 {
    let r_n = g.len();
    let c_n = if r_n > 0 { g[0].len() } else { 0 };
    let mut per = 0;
    for r in 0..r_n {
        for c in 0..c_n {
            if g[r][c] == 1 {
                per += 4;
                if c + 1 < c_n && g[r][c + 1] == 1 { per -= 2; }
                if r + 1 < r_n && g[r + 1][c] == 1 { per -= 2; }
            }
        }
    }
    per
}

fn main() {
    let grid = vec![
        vec![0, 1, 1, 0],
        vec![1, 1, 1, 0],
        vec![0, 1, 1, 0],
        vec![0, 0, 1, 0],
    ];
    println!("{}", island_perimeter(&grid));
}
