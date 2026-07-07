// Build origin->sorted destinations; DFS backtracking trying lexicographically
// smallest dest first; first path using all flights is the answer.
// Time O(E!) worst case, Space O(E).
use std::collections::BTreeMap;

fn find_itinerary(flights: &[(&str, &str)], start: &str) -> Option<Vec<String>> {
    let total = flights.len();
    let mut graph: BTreeMap<String, Vec<Option<String>>> = BTreeMap::new();
    for (o, d) in flights {
        graph.entry(o.to_string()).or_default().push(Some(d.to_string()));
    }
    for v in graph.values_mut() {
        v.sort();
    }

    fn dfs(node: &str, path: &mut Vec<String>, total: usize,
           graph: &mut BTreeMap<String, Vec<Option<String>>>) -> bool {
        if path.len() == total + 1 {
            return true;
        }
        let len = graph.get(node).map_or(0, |v| v.len());
        for i in 0..len {
            let d = match graph.get(node).and_then(|v| v[i].clone()) {
                Some(d) => d,
                None => continue,
            };
            graph.get_mut(node).unwrap()[i] = None;
            path.push(d.clone());
            if dfs(&d, path, total, graph) {
                return true;
            }
            path.pop();
            graph.get_mut(node).unwrap()[i] = Some(d);
        }
        false
    }

    let mut path = vec![start.to_string()];
    if dfs(start, &mut path, total, &mut graph) {
        Some(path)
    } else {
        None
    }
}

fn show(r: Option<Vec<String>>) {
    match r {
        None => println!("null"),
        Some(v) => {
            let parts: Vec<String> = v.iter().map(|s| format!("'{}'", s)).collect();
            println!("[{}]", parts.join(", "));
        }
    }
}

fn main() {
    show(find_itinerary(&[("SFO","HKO"),("YYZ","SFO"),("YUL","YYZ"),("HKO","ORD")], "YUL"));
    show(find_itinerary(&[("SFO","COM"),("COM","YYZ")], "COM"));
    show(find_itinerary(&[("A","B"),("A","C"),("B","C"),("C","A")], "A"));
}
