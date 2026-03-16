// Day 1214: Reconstruct itinerary using all flights, lexicographically smallest.
// Hierholzer's Eulerian path over sorted multigraph adjacency. Time O(E log E), Space O(E).
use std::collections::HashMap;

fn find_itinerary(flights: &[(&str, &str)], start: &str) -> Option<Vec<String>> {
    let mut adj: HashMap<String, Vec<String>> = HashMap::new();
    for &(u, v) in flights {
        adj.entry(u.to_string()).or_default().push(v.to_string());
    }
    for list in adj.values_mut() {
        list.sort();
        list.reverse(); // pop() yields the smallest
    }
    let total = flights.len();
    let mut route: Vec<String> = Vec::new();
    let mut st: Vec<String> = vec![start.to_string()];
    while let Some(u) = st.last().cloned() {
        if let Some(list) = adj.get_mut(&u) {
            if let Some(v) = list.pop() {
                st.push(v);
                continue;
            }
        }
        route.push(st.pop().unwrap());
    }
    route.reverse();
    if route.len() == total + 1 {
        Some(route)
    } else {
        None
    }
}

fn main() {
    let flights = [("SFO", "HKO"), ("YYZ", "SFO"), ("YUL", "YYZ"), ("HKO", "ORD")];
    println!("{:?}", find_itinerary(&flights, "YUL"));
    // Some(["YUL", "YYZ", "SFO", "HKO", "ORD"])
}
