// Skyline via sweep-line + BTreeMap multiset of active heights. Emit point when max changes.
// Time: O(n log n), Space: O(n).
use std::collections::BTreeMap;

fn get_skyline(buildings: &[(i32, i32, i32)]) -> Vec<(i32, i32)> {
    // events: (x, signed height) with start = -h, end = +h
    let mut events: Vec<(i32, i32)> = Vec::new();
    for &(left, right, h) in buildings {
        events.push((left, -h));
        events.push((right, h));
    }
    events.sort();

    let mut heights: BTreeMap<i32, i32> = BTreeMap::new();
    heights.insert(0, 1);
    let mut prev = 0;
    let mut res: Vec<(i32, i32)> = Vec::new();
    for &(x, h) in &events {
        if h < 0 {
            *heights.entry(-h).or_insert(0) += 1;
        } else {
            if let Some(c) = heights.get_mut(&h) {
                *c -= 1;
                if *c == 0 {
                    heights.remove(&h);
                }
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
    let buildings = [(0, 15, 3), (4, 11, 5), (19, 23, 4)];
    let res = get_skyline(&buildings);
    let parts: Vec<String> = res.iter().map(|&(x, h)| format!("({}, {})", x, h)).collect();
    println!("[{}]", parts.join(", "));
}
