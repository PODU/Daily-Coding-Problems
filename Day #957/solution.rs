// Day 957: skyline problem via sweep line with a BTreeMap of active heights.
// Time O(n log n), Space O(n).
use std::collections::BTreeMap;

fn skyline(buildings: &[(i32, i32, i32)]) -> Vec<(i32, i32)> {
    let mut events: Vec<(i32, i32)> = Vec::new();
    for &(l, r, h) in buildings {
        events.push((l, -h)); // start
        events.push((r, h));  // end
    }
    events.sort();
    let mut active: BTreeMap<i32, i32> = BTreeMap::new();
    *active.entry(0).or_insert(0) += 1; // baseline
    let mut prev = 0;
    let mut res = Vec::new();
    for (x, h) in events {
        if h < 0 {
            *active.entry(-h).or_insert(0) += 1;
        } else {
            let c = active.get_mut(&h).unwrap();
            *c -= 1;
            if *c == 0 {
                active.remove(&h);
            }
        }
        let cur = *active.keys().next_back().unwrap();
        if cur != prev {
            res.push((x, cur));
            prev = cur;
        }
    }
    res
}

fn main() {
    let res = skyline(&[(0, 15, 3), (4, 11, 5), (19, 23, 4)]);
    let parts: Vec<String> = res.iter().map(|(x, h)| format!("({}, {})", x, h)).collect();
    println!("[{}]", parts.join(", "));
    // [(0, 3), (4, 5), (11, 3), (15, 0), (19, 4), (23, 0)]
}
