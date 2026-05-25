// Greedy interval stabbing: sort by right endpoint; for the smallest uncovered right,
// pick point = max left among intervals containing it, cover them all. Time O(n^2), Space O(n).
fn stab_points(intervals: &[(i32, i32)]) -> Vec<i32> {
    let mut iv = intervals.to_vec();
    iv.sort_by_key(|x| x.1);
    let n = iv.len();
    let mut covered = vec![false; n];
    let mut points = Vec::new();
    for i in 0..n {
        if covered[i] {
            continue;
        }
        let r = iv[i].1;
        let mut point = i32::MIN;
        for j in i..n {
            if !covered[j] && iv[j].0 <= r {
                point = point.max(iv[j].0);
            }
        }
        points.push(point);
        for j in i..n {
            if !covered[j] && iv[j].0 <= point && point <= iv[j].1 {
                covered[j] = true;
            }
        }
    }
    points
}

fn main() {
    let pts = stab_points(&[(0, 3), (2, 6), (3, 4), (6, 9)]);
    let strs: Vec<String> = pts.iter().map(|p| p.to_string()).collect();
    println!("{{{}}}", strs.join(", "));
}
