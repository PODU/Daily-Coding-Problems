// Day 969: Count islands (4-connected groups of 1s) in a binary matrix.
// Approach: DFS flood fill from each unvisited land cell. Time O(R*C), Space O(R*C).

fn dfs(g: &mut Vec<Vec<i32>>, r: i32, c: i32) {
    if r < 0 || r >= g.len() as i32 || c < 0 || c >= g[0].len() as i32 || g[r as usize][c as usize] == 0 {
        return;
    }
    g[r as usize][c as usize] = 0;
    dfs(g, r + 1, c);
    dfs(g, r - 1, c);
    dfs(g, r, c + 1);
    dfs(g, r, c - 1);
}

fn num_islands(grid: &[Vec<i32>]) -> i32 {
    let mut g: Vec<Vec<i32>> = grid.iter().cloned().collect();
    let mut count = 0;
    for i in 0..g.len() {
        for j in 0..g[0].len() {
            if g[i][j] == 1 {
                count += 1;
                dfs(&mut g, i as i32, j as i32);
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
    println!("{}", num_islands(&grid)); // 4
}
