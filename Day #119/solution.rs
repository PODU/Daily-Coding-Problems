// Day 119: Min points to stab all intervals. Greedy: sort by start desc, pick start
// of each not-yet-stabbed interval (mirror of the classic sort-by-end greedy). O(n log n).
fn min_cover(mut iv: Vec<(i32, i32)>) -> Vec<i32> {
    iv.sort_by(|a, b| b.0.cmp(&a.0));
    let mut pts = Vec::new();
    let mut last: Option<i32> = None;
    for (s, e) in iv {
        if last.is_none() || last.unwrap() > e {
            last = Some(s);
            pts.push(s);
        }
    }
    pts.sort();
    pts
}

fn main() {
    let r = min_cover(vec![(0, 3), (2, 6), (3, 4), (6, 9)]);
    let strs: Vec<String> = r.iter().map(|x| x.to_string()).collect();
    println!("{{{}}}", strs.join(", ")); // {3, 6}
}
