// Celebrity problem: O(N) elimination to one candidate, then O(N) verify.
fn knows(m: &[Vec<i32>], a: usize, b: usize) -> bool {
    m[a][b] == 1
}

fn find_celebrity(m: &[Vec<i32>], n: usize) -> i64 {
    let mut cand = 0usize;
    for i in 1..n {
        if knows(m, cand, i) {
            cand = i;
        }
    }
    for i in 0..n {
        if i != cand && (knows(m, cand, i) || !knows(m, i, cand)) {
            return -1;
        }
    }
    cand as i64
}

fn main() {
    let m = vec![
        vec![0, 1, 1, 0],
        vec![0, 0, 1, 0],
        vec![0, 0, 0, 0],
        vec![0, 1, 1, 0],
    ];
    println!("{}", find_celebrity(&m, 4));
}
