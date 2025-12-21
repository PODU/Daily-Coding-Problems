// Day 778: Interleave ranked preference lists -> topological sort (Kahn's).
// Consecutive items in each list become edges. FIFO queue + first-seen order.
// O(V + E).
use std::collections::{HashMap, HashSet, VecDeque};

fn interleave(lists: &[Vec<i32>]) -> Vec<i32> {
    let mut order: Vec<i32> = Vec::new();
    let mut seen: HashSet<i32> = HashSet::new();
    let mut adj: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut indeg: HashMap<i32, i32> = HashMap::new();
    for l in lists {
        for &x in l {
            if seen.insert(x) {
                order.push(x);
                indeg.entry(x).or_insert(0);
            }
        }
        for w in l.windows(2) {
            adj.entry(w[0]).or_default().push(w[1]);
            *indeg.entry(w[1]).or_insert(0) += 1;
        }
    }
    let mut q: VecDeque<i32> = order.iter().cloned().filter(|x| indeg[x] == 0).collect();
    let mut res = Vec::new();
    while let Some(u) = q.pop_front() {
        res.push(u);
        if let Some(neis) = adj.get(&u) {
            for &v in neis {
                let e = indeg.get_mut(&v).unwrap();
                *e -= 1;
                if *e == 0 {
                    q.push_back(v);
                }
            }
        }
    }
    res
}

fn main() {
    let lists = vec![vec![1, 7, 3], vec![2, 1, 6, 7, 9], vec![3, 9, 5]];
    println!("{:?}", interleave(&lists)); // [2, 1, 6, 7, 3, 9, 5]
}
