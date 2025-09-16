// Day 286: Skyline problem.
// Sweep line over events; a BTreeMap acts as a multiset of active heights.
// Time: O(n log n), Space: O(n).
use std::collections::BTreeMap;

fn get_skyline(buildings: &[(i32, i32, i32)]) -> Vec<(i32, i32)> {
    let mut events: Vec<(i32, i32)> = Vec::new();
    for &(l, r, h) in buildings {
        events.push((l, -h));
        events.push((r, h));
    }
    events.sort();
    let mut live: BTreeMap<i32, i32> = BTreeMap::new();
    live.insert(0, 1);
    let mut prev = 0;
    let mut res: Vec<(i32, i32)> = Vec::new();
    for (x, h) in events {
        if h < 0 {
            *live.entry(-h).or_insert(0) += 1;
        } else {
            let c = live.get_mut(&h).unwrap();
            *c -= 1;
            if *c == 0 {
                live.remove(&h);
            }
        }
        let cur = *live.keys().next_back().unwrap();
        if cur != prev {
            res.push((x, cur));
            prev = cur;
        }
    }
    res
}

fn main() {
    let buildings = vec![(0, 15, 3), (4, 11, 5), (19, 23, 4)];
    let sky = get_skyline(&buildings);
    let parts: Vec<String> = sky.iter().map(|p| format!("({}, {})", p.0, p.1)).collect();
    println!("[{}]", parts.join(", "));
}
