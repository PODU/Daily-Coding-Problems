// Celebrity problem: one candidate via elimination, then verify.
// Two-pointer elimination + verification. Time: O(N) knows calls, Space: O(1).

// Demo knows matrix: N=4, person 2 is the celebrity.
const M: [[i32; 4]; 4] = [
    [0, 1, 1, 0], // 0 knows 2
    [0, 0, 1, 0], // 1 knows 2
    [0, 0, 0, 0], // 2 (celebrity) knows no one
    [0, 1, 1, 0], // 3 knows 2
];

fn knows(a: usize, b: usize) -> bool {
    M[a][b] == 1
}

fn find_celebrity(n: usize) -> i32 {
    let mut cand = 0usize;
    for i in 1..n {
        if knows(cand, i) {
            cand = i;
        }
    }
    for i in 0..n {
        if i == cand {
            continue;
        }
        if knows(cand, i) || !knows(i, cand) {
            return -1;
        }
    }
    cand as i32
}

fn main() {
    println!("{}", find_celebrity(M.len()));
}
