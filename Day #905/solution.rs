// Word-chain circle = Eulerian circuit on directed graph (node=letter, edge first->last).
// Check in==out degree for all nodes and single SCC over non-zero-degree nodes. O(V+E).
fn dfs(g: &Vec<Vec<usize>>, u: usize, vis: &mut [bool; 26]) {
    vis[u] = true;
    for &v in &g[u] {
        if !vis[v] {
            dfs(g, v, vis);
        }
    }
}

fn can_chain(words: &[&str]) -> bool {
    let mut indeg = [0i32; 26];
    let mut outdeg = [0i32; 26];
    let mut adj: Vec<Vec<usize>> = vec![Vec::new(); 26];
    let mut radj: Vec<Vec<usize>> = vec![Vec::new(); 26];
    for w in words {
        let a = (w.bytes().next().unwrap() - b'a') as usize;
        let b = (w.bytes().last().unwrap() - b'a') as usize;
        outdeg[a] += 1;
        indeg[b] += 1;
        adj[a].push(b);
        radj[b].push(a);
    }
    for i in 0..26 {
        if indeg[i] != outdeg[i] {
            return false;
        }
    }
    let mut start = None;
    for i in 0..26 {
        if outdeg[i] > 0 {
            start = Some(i);
            break;
        }
    }
    let start = match start {
        Some(s) => s,
        None => return true,
    };
    let mut vis = [false; 26];
    dfs(&adj, start, &mut vis);
    for i in 0..26 {
        if (indeg[i] != 0 || outdeg[i] != 0) && !vis[i] {
            return false;
        }
    }
    let mut vis = [false; 26];
    dfs(&radj, start, &mut vis);
    for i in 0..26 {
        if (indeg[i] != 0 || outdeg[i] != 0) && !vis[i] {
            return false;
        }
    }
    true
}

fn main() {
    let words = ["chair", "height", "racket", "touch", "tunic"];
    println!("{}", if can_chain(&words) { "True" } else { "False" });
}
