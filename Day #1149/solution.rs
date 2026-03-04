// Day 1149: Number of islands - 4-directional flood fill.
// Iterative DFS marks visited land. Time O(R*C), Space O(R*C).
fn num_islands(grid: &[Vec<i32>]) -> i32 {
    let rows = grid.len();
    if rows == 0 {
        return 0;
    }
    let cols = grid[0].len();
    let mut g: Vec<Vec<i32>> = grid.to_vec();
    let dirs = [(1i32, 0i32), (-1, 0), (0, 1), (0, -1)];
    let mut count = 0;
    for r in 0..rows {
        for c in 0..cols {
            if g[r][c] == 1 {
                count += 1;
                let mut stack = vec![(r as i32, c as i32)];
                g[r][c] = 0;
                while let Some((x, y)) = stack.pop() {
                    for (dx, dy) in dirs.iter() {
                        let nx = x + dx;
                        let ny = y + dy;
                        if nx >= 0 && ny >= 0 && (nx as usize) < rows && (ny as usize) < cols
                            && g[nx as usize][ny as usize] == 1
                        {
                            g[nx as usize][ny as usize] = 0;
                            stack.push((nx, ny));
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
        vec![1, 0, 0, 0, 0], vec![0, 0, 1, 1, 0], vec![0, 1, 1, 0, 0],
        vec![0, 0, 0, 0, 0], vec![1, 1, 0, 0, 1], vec![1, 1, 0, 0, 1]];
    println!("{}", num_islands(&grid)); // 4
}
