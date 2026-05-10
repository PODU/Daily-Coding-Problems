// Day 1500: Number of islands via iterative DFS flood fill (4-directional).
// Time O(R*C), Space O(R*C).
fn num_islands(grid: &Vec<Vec<i32>>) -> i32 {
    let rows = grid.len();
    if rows == 0 {
        return 0;
    }
    let cols = grid[0].len();
    let mut g = grid.clone();
    let dirs = [(1i32, 0i32), (-1, 0), (0, 1), (0, -1)];
    let mut count = 0;
    for sr in 0..rows {
        for sc in 0..cols {
            if g[sr][sc] == 1 {
                count += 1;
                let mut stack = vec![(sr as i32, sc as i32)];
                g[sr][sc] = 0;
                while let Some((r, c)) = stack.pop() {
                    for (dr, dc) in dirs.iter() {
                        let nr = r + dr;
                        let nc = c + dc;
                        if nr >= 0 && nr < rows as i32 && nc >= 0 && nc < cols as i32
                            && g[nr as usize][nc as usize] == 1
                        {
                            g[nr as usize][nc as usize] = 0;
                            stack.push((nr, nc));
                        }
                    }
                }
            }
        }
    }
    count
}

fn main() {
    let grid = vec![
        vec![1, 0, 0, 0, 0],
        vec![0, 0, 1, 1, 0],
        vec![0, 1, 1, 0, 0],
        vec![0, 0, 0, 0, 0],
        vec![1, 1, 0, 0, 1],
        vec![1, 1, 0, 0, 1],
    ];
    println!("{}", num_islands(&grid));
}
