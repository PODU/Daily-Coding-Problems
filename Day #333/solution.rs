// Celebrity problem: 2-pass. Pass 1 eliminate to one candidate; pass 2 verify.
// Time O(n), Space O(1).
const M: [[i32; 4]; 4] = [
    [0, 1, 1, 0], // person0 knows {1,2}
    [0, 0, 1, 0], // person1 knows {2}
    [0, 0, 0, 0], // person2 knows {}
    [1, 1, 1, 0], // person3 knows {0,1,2}
];

fn knows(a: usize, b: usize) -> bool {
    M[a][b] == 1
}

fn find_celebrity(n: usize) -> i32 {
    let mut cand = 0;
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
