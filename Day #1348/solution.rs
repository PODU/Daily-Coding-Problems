// Weighted tree diameter: DFS, at each node track two largest downward child path sums;
// diameter = max over nodes of (sum of two largest). Time O(V+E), Space O(V+E).

fn dfs(u: usize, parent: i32, adj: &Vec<Vec<(usize, i32)>>, best: &mut i32) -> i32 {
    let mut max1 = 0;
    let mut max2 = 0; // two largest downward path sums
    for &(v, w) in &adj[u] {
        if v as i32 == parent {
            continue;
        }
        let down = dfs(v, u as i32, adj, best) + w;
        if down > max1 {
            max2 = max1;
            max1 = down;
        } else if down > max2 {
            max2 = down;
        }
    }
    *best = (*best).max(max1 + max2);
    max1
}

fn main() {
    let n = 8; // a..h -> 0..7
    let mut adj: Vec<Vec<(usize, i32)>> = vec![Vec::new(); n];
    let mut add = |a: usize, b: usize, w: i32, adj: &mut Vec<Vec<(usize, i32)>>| {
        adj[a].push((b, w));
        adj[b].push((a, w));
    };
    add(0, 1, 3, &mut adj); // a-b
    add(0, 2, 5, &mut adj); // a-c
    add(0, 3, 8, &mut adj); // a-d
    add(3, 4, 2, &mut adj); // d-e
    add(3, 5, 4, &mut adj); // d-f
    add(4, 6, 1, &mut adj); // e-g
    add(4, 7, 1, &mut adj); // e-h
    let mut best = 0;
    dfs(0, -1, &adj, &mut best);
    println!("{}", best);
}
