// Day 92: Topological sort (Kahn's algorithm) over a prerequisite graph.
// Returns a valid course order or None if a cycle exists. O(V+E).
use std::collections::{BTreeMap, BTreeSet};

fn course_order(prereqs: &BTreeMap<String, Vec<String>>) -> Option<Vec<String>> {
    let mut indeg: BTreeMap<String, i32> = BTreeMap::new();
    let mut adj: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for c in prereqs.keys() {
        indeg.entry(c.clone()).or_insert(0);
    }
    for (c, pres) in prereqs {
        for p in pres {
            adj.entry(p.clone()).or_default().push(c.clone());
            *indeg.entry(c.clone()).or_insert(0) += 1;
        }
    }
    let mut ready: BTreeSet<String> =
        indeg.iter().filter(|(_, &d)| d == 0).map(|(c, _)| c.clone()).collect();
    let mut order = Vec::new();
    while let Some(c) = ready.iter().next().cloned() {
        ready.remove(&c);
        order.push(c.clone());
        if let Some(nexts) = adj.get(&c) {
            for n in nexts.clone() {
                let e = indeg.get_mut(&n).unwrap();
                *e -= 1;
                if *e == 0 {
                    ready.insert(n);
                }
            }
        }
    }
    if order.len() == prereqs.len() { Some(order) } else { None }
}

fn main() {
    let mut g: BTreeMap<String, Vec<String>> = BTreeMap::new();
    g.insert("CSC300".into(), vec!["CSC100".into(), "CSC200".into()]);
    g.insert("CSC200".into(), vec!["CSC100".into()]);
    g.insert("CSC100".into(), vec![]);
    println!("{:?}", course_order(&g)); // Some(["CSC100", "CSC200", "CSC300"])
}
