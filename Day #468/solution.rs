// Rotate NxN matrix 90 deg clockwise in place: transpose then reverse each row.
// Time: O(n^2), Space: O(1).
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
    let n = m.len();
    let mut lines: Vec<String> = Vec::new();
    for i in 0..n {
        let prefix = if i == 0 { "[[" } else { " [" };
        let body: Vec<String> = m[i].iter().map(|x| x.to_string()).collect();
        let suffix = if i == n - 1 { "]]" } else { "]" };
        lines.push(format!("{}{}{}", prefix, body.join(", "), suffix));
    }
    println!("{}", lines.join(",\n"));
}
