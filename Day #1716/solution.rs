// Day 1716: Print an N x M matrix in clockwise spiral order.
// Boundary-shrinking traversal. Time: O(N*M), Space: O(1) extra.

fn spiral(m: &Vec<Vec<i32>>) {
    if m.is_empty() || m[0].is_empty() {
        return;
    }
    let mut top: i32 = 0;
    let mut bottom: i32 = m.len() as i32 - 1;
    let mut left: i32 = 0;
    let mut right: i32 = m[0].len() as i32 - 1;
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
            let mut c = right;
            while c >= left {
                println!("{}", m[bottom as usize][c as usize]);
                c -= 1;
            }
            bottom -= 1;
        }
        if left <= right {
            let mut r = bottom;
            while r >= top {
                println!("{}", m[r as usize][left as usize]);
                r -= 1;
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
