// Day 971: Rotate N x N matrix 90 degrees clockwise in place.
// Approach: transpose then reverse each row. Time O(N^2), Space O(1).

fn rotate(m: &mut Vec<Vec<i32>>) {
    let n = m.len();
    for i in 0..n {
        for j in (i + 1)..n {
            let t = m[i][j];
            m[i][j] = m[j][i];
            m[j][i] = t;
        }
    }
    for row in m.iter_mut() {
        row.reverse();
    }
}

fn main() {
    let mut m = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    rotate(&mut m);
    for row in &m {
        let s: Vec<String> = row.iter().map(|v| v.to_string()).collect();
        println!("{}", s.join(" "));
    }
    // 7 4 1 / 8 5 2 / 9 6 3
}
