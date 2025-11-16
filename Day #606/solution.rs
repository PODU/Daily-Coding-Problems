// Count all open knight's tours: DFS backtracking from every start square,
// counting Hamiltonian paths. Time O(8^(N*N)) worst, Space O(N*N). N=5 -> 1728.
const MOVES: [(i32, i32); 8] = [
    (1, 2), (1, -2), (-1, 2), (-1, -2),
    (2, 1), (2, -1), (-2, 1), (-2, -1),
];

fn dfs(x: i32, y: i32, visited: usize, n: i32, board: &mut Vec<Vec<bool>>) -> u64 {
    if visited == (n * n) as usize {
        return 1;
    }
    let mut count = 0u64;
    for &(dx, dy) in MOVES.iter() {
        let nx = x + dx;
        let ny = y + dy;
        if nx >= 0 && nx < n && ny >= 0 && ny < n && !board[nx as usize][ny as usize] {
            board[nx as usize][ny as usize] = true;
            count += dfs(nx, ny, visited + 1, n, board);
            board[nx as usize][ny as usize] = false;
        }
    }
    count
}

fn knight_tours(n: i32) -> u64 {
    if n == 0 {
        return 0;
    }
    let mut total = 0u64;
    for i in 0..n {
        for j in 0..n {
            let mut board = vec![vec![false; n as usize]; n as usize];
            board[i as usize][j as usize] = true;
            total += dfs(i, j, 1, n, &mut board);
        }
    }
    total
}

fn main() {
    println!("{}", knight_tours(5));
}
