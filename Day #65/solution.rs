// Spiral order print via boundary shrinking (top/bottom/left/right). Time O(N*M), Space O(1) extra.
fn main() {
    let m = vec![
        vec![1, 2, 3, 4, 5],
        vec![6, 7, 8, 9, 10],
        vec![11, 12, 13, 14, 15],
        vec![16, 17, 18, 19, 20],
    ];
    let mut top: i32 = 0;
    let mut bottom: i32 = m.len() as i32 - 1;
    let mut left: i32 = 0;
    let mut right: i32 = m[0].len() as i32 - 1;
    while top <= bottom && left <= right {
        for j in left..=right {
            println!("{}", m[top as usize][j as usize]);
        }
        top += 1;
        for i in top..=bottom {
            println!("{}", m[i as usize][right as usize]);
        }
        right -= 1;
        if top <= bottom {
            let mut j = right;
            while j >= left {
                println!("{}", m[bottom as usize][j as usize]);
                j -= 1;
            }
            bottom -= 1;
        }
        if left <= right {
            let mut i = bottom;
            while i >= top {
                println!("{}", m[i as usize][left as usize]);
                i -= 1;
            }
            left += 1;
        }
    }
}
