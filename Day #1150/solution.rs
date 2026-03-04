// Day 1150: Skyline - sweep line over building edges with a height multiset (BTreeMap).
// Track current max height; emit point when it changes. Time O(n log n), Space O(n).
use std::collections::BTreeMap;

fn skyline(buildings: &[(i32, i32, i32)]) -> Vec<(i32, i32)> {
    let mut events: Vec<(i32, i32)> = Vec::new();
    for &(l, r, h) in buildings {
        events.push((l, -h)); // start
        events.push((r, h));  // end
    }
    events.sort();
    let mut heights: BTreeMap<i32, i32> = BTreeMap::new();
    *heights.entry(0).or_insert(0) += 1;
    let mut prev = 0;
    let mut res = Vec::new();
    for &(x, h) in &events {
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
    let bld = [(0, 15, 3), (4, 11, 5), (19, 23, 4)];
    let sk = skyline(&bld);
    let parts: Vec<String> = sk.iter().map(|(x, h)| format!("({}, {})", x, h)).collect();
    println!("[{}]", parts.join(", "));
    // [(0, 3), (4, 5), (11, 3), (15, 0), (19, 4), (23, 0)]
}
