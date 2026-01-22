// Day 939: Print an N x M matrix in clockwise spiral order.
// Shrink four boundaries layer by layer. Time O(N*M), Space O(1) extra.
fn spiral(m: &[Vec<i32>]) {
    if m.is_empty() {
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
    let matrix = vec![
        vec![1, 2, 3, 4, 5],
        vec![6, 7, 8, 9, 10],
        vec![11, 12, 13, 14, 15],
        vec![16, 17, 18, 19, 20],
    ];
    spiral(&matrix); // 1 2 3 4 5 10 15 20 19 18 17 16 11 6 7 8 9 14 13 12
}
