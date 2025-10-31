// Spiral traversal by peeling outer ring (top,right,bottom,left) toward center.
// Time O(N*M), Space O(1) extra.
fn spiral(m: &[Vec<i32>]) {
    if m.is_empty() {
        return;
    }
    let (mut top, mut bottom, mut left, mut right) =
        (0i32, m.len() as i32 - 1, 0i32, m[0].len() as i32 - 1);
    while top <= bottom && left <= right {
        for c in left..=right {
            println!("{}", m[top as usize][c as usize]);
        }
        top += 1;
        for r in top..=bottom {
            println!("{}", m[r as usize][right as usize]);
        }
        right -= 1;
        if top <= bottom {
            for c in (left..=right).rev() {
                println!("{}", m[bottom as usize][c as usize]);
            }
            bottom -= 1;
        }
        if left <= right {
            for r in (top..=bottom).rev() {
                println!("{}", m[r as usize][left as usize]);
            }
            left += 1;
        }
    }
}

fn main() {
    let m = vec![
        vec![1, 2, 3, 4, 5],
        vec![6, 7, 8, 9, 10],
        vec![11, 12, 13, 14, 15],
        vec![16, 17, 18, 19, 20],
    ];
    spiral(&m);
}
