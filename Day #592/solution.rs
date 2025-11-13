// Day 592: Count islands in a binary matrix.
// Approach: iterative DFS flood-fill over each unvisited land cell (4-directional).
// Time: O(R*C), Space: O(R*C).
fn num_islands(grid: &Vec<Vec<i32>>) -> i32 {
    let mut g: Vec<Vec<i32>> = grid.clone();
    let r = g.len();
    let c = g[0].len();
    let mut count = 0;
    for sr in 0..r {
        for sc in 0..c {
            if g[sr][sc] == 1 {
                count += 1;
                let mut stack = vec![(sr as i32, sc as i32)];
                g[sr][sc] = 0;
                while let Some((cr, cc)) = stack.pop() {
                    for (dr, dc) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                        let nr = cr + dr;
                        let nc = cc + dc;
                        if nr >= 0 && nr < r as i32 && nc >= 0 && nc < c as i32
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
    println!("{}", num_islands(&grid)); // 4
}
