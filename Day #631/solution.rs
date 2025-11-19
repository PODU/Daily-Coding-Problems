// Day 631: Skyline problem.
// Approach: sweep line over edges + BTreeMap multiset of active heights.
// Time: O(n log n), Space: O(n).
use std::collections::BTreeMap;

fn get_skyline(buildings: &[(i32, i32, i32)]) -> Vec<(i32, i32)> {
    let mut events: Vec<(i32, i32)> = Vec::new();
    for &(l, r, h) in buildings {
        events.push((l, -h)); // start
        events.push((r, h));  // end
    }
    events.sort();
    let mut heights: BTreeMap<i32, i32> = BTreeMap::new();
    heights.insert(0, 1);
    let mut prev = 0;
    let mut res = Vec::new();
    for (x, h) in events {
        if h < 0 {
            *heights.entry(-h).or_insert(0) += 1;
        } else {
            let c = heights.get_mut(&h).unwrap();
            *c -= 1;
            if *c == 0 {
                heights.remove(&h);
            }
        }
        let cur = *heights.keys().next_back().unwrap();
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
    let parts: Vec<String> = sky.iter().map(|(x, h)| format!("({}, {})", x, h)).collect();
    println!("[{}]", parts.join(", "));
}
