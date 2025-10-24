// Day 486: Celebrity problem.
// Two-pointer elimination: one candidate survives in O(n) knows() calls, then
// verify in O(n). Total O(n) time, O(1) space.

// mock relation matrix: knows[a][b] == 1 means a knows b
fn knows(matrix: &[Vec<i32>], a: usize, b: usize) -> bool {
    matrix[a][b] == 1
}

fn find_celebrity(matrix: &[Vec<i32>], n: usize) -> i32 {
    let mut candidate = 0usize;
    for i in 1..n {
        if knows(matrix, candidate, i) {
            candidate = i;
        }
    }
    for i in 0..n {
        if i == candidate {
            continue;
        }
        if knows(matrix, candidate, i) || !knows(matrix, i, candidate) {
            return -1;
        }
    }
    candidate as i32
}

fn main() {
    // person 2 is the celebrity: knows nobody, everyone knows them
    let matrix = vec![
        vec![0, 1, 1, 0],
        vec![1, 0, 1, 1],
        vec![0, 0, 0, 0],
        vec![1, 1, 1, 0],
    ];
    println!("{}", find_celebrity(&matrix, 4)); // 2
}
