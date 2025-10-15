// Day 439: Reconstruct itinerary using all flights, lexicographically smallest.
// Hierholzer's Eulerian-path algorithm with sorted adjacency. O(E log E).
use std::collections::HashMap;

fn find_itinerary(flights: &[(&str, &str)], start: &str) -> Option<Vec<String>> {
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();
    for &(o, d) in flights {
        graph.entry(o.to_string()).or_default().push(d.to_string());
    }
    // sort descending so pop() yields the lexicographically smallest next hop
    for list in graph.values_mut() {
        list.sort_by(|a, b| b.cmp(a));
    }

    let mut route: Vec<String> = Vec::new();
    let mut stack: Vec<String> = vec![start.to_string()];
    while let Some(u) = stack.last().cloned() {
        let has_next = graph.get(&u).map_or(false, |l| !l.is_empty());
        if has_next {
            let next = graph.get_mut(&u).unwrap().pop().unwrap();
            stack.push(next);
        } else {
            route.push(stack.pop().unwrap());
        }
    }
    route.reverse();
    if route.len() != flights.len() + 1 {
        return None;
    }
    Some(route)
}

fn main() {
    let flights = [("SFO", "HKO"), ("YYZ", "SFO"), ("YUL", "YYZ"), ("HKO", "ORD")];
    println!("{:?}", find_itinerary(&flights, "YUL"));
    // Some(["YUL", "YYZ", "SFO", "HKO", "ORD"])
    println!("{:?}", find_itinerary(&[("SFO", "COM"), ("COM", "YYZ")], "COM")); // None
    println!("{:?}", find_itinerary(&[("A", "B"), ("A", "C"), ("B", "C"), ("C", "A")], "A"));
    // Some(["A", "B", "C", "A", "C"])
}
