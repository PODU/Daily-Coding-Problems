// Celebrity finder: two-phase candidate elimination then verify. O(N) knows() calls, O(1) space.

const KNOWS_MAT: [[i32; 4]; 4] = [
    [0, 1, 1, 0],
    [0, 0, 1, 0],
    [0, 0, 0, 0],
    [0, 1, 1, 0],
];

fn knows(a: usize, b: usize) -> bool {
    KNOWS_MAT[a][b] == 1
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
    println!("{}", find_celebrity(4));
}
