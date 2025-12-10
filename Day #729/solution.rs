// Day 729: Interleave ranked preference lists into one consistent playlist.
// Approach: Build precedence DAG (consecutive pairs), Kahn topological sort (FIFO,
// first-appearance tie-break). Time: O(V + E), Space: O(V + E).
use std::collections::{HashMap, HashSet, VecDeque};

fn interleave(lists: &[Vec<i32>]) -> Vec<i32> {
    let mut adj: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut indeg: HashMap<i32, i32> = HashMap::new();
    let mut edges: HashSet<(i32, i32)> = HashSet::new();
    let mut order: Vec<i32> = Vec::new();
    for lst in lists {
        for &x in lst {
            indeg.entry(x).or_insert_with(|| { adj.insert(x, Vec::new()); order.push(x); 0 });
        }
        for w in lst.windows(2) {
            let (a, b) = (w[0], w[1]);
            if edges.insert((a, b)) {
                adj.get_mut(&a).unwrap().push(b);
                *indeg.get_mut(&b).unwrap() += 1;
            }
        }
    }
    let mut q: VecDeque<i32> = order.iter().cloned().filter(|x| indeg[x] == 0).collect();
    let mut res = Vec::new();
    while let Some(u) = q.pop_front() {
        res.push(u);
        let neighbors = adj[&u].clone();
        for v in neighbors {
            let d = indeg.get_mut(&v).unwrap();
            *d -= 1;
            if *d == 0 { q.push_back(v); }
        }
    }
    res
}

fn main() {
    let lists = vec![vec![1, 7, 3], vec![2, 1, 6, 7, 9], vec![3, 9, 5]];
    println!("{:?}", interleave(&lists));
}
