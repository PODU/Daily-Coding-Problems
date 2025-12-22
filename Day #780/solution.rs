// Day 780: Topological sort of courses (prereqs map). DFS post-order with
// cycle detection; returns None if a cycle exists. O(V + E).
use std::collections::HashMap;

fn dfs(c: &str, g: &HashMap<String, Vec<String>>, state: &mut HashMap<String, i32>, order: &mut Vec<String>) -> bool {
    state.insert(c.to_string(), 1);
    if let Some(pres) = g.get(c) {
        for pre in pres {
            match state.get(pre).copied().unwrap_or(0) {
                1 => return false,
                0 => {
                    if !dfs(pre, g, state, order) {
                        return false;
                    }
                }
                _ => {}
            }
        }
    }
    state.insert(c.to_string(), 2);
    order.push(c.to_string());
    true
}

fn course_order(g: &HashMap<String, Vec<String>>) -> Option<Vec<String>> {
    let mut keys: Vec<&String> = g.keys().collect();
    keys.sort();
    let mut state: HashMap<String, i32> = HashMap::new();
    let mut order: Vec<String> = Vec::new();
    for c in keys {
        if state.get(c).copied().unwrap_or(0) == 0 && !dfs(c, g, &mut state, &mut order) {
            return None;
        }
    }
    Some(order)
}

fn main() {
    let mut g: HashMap<String, Vec<String>> = HashMap::new();
    g.insert("CSC300".into(), vec!["CSC100".into(), "CSC200".into()]);
    g.insert("CSC200".into(), vec!["CSC100".into()]);
    g.insert("CSC100".into(), vec![]);
    match course_order(&g) {
        Some(order) => println!("['{}']", order.join("', '")), // ['CSC100', 'CSC200', 'CSC300']
        None => println!("null"),
    }
}
