// Day 603: Final resting state of pushed dominoes.
// Approach: two force passes (R rightward, L leftward), compare. Time O(n), Space O(n).

fn push_dominoes(s: &str) -> String {
    let b = s.as_bytes();
    let n = b.len();
    let mut fr = vec![0i32; n];
    let mut fl = vec![0i32; n];
    let mut f = 0i32;
    for i in 0..n {
        match b[i] {
            b'R' => f = n as i32,
            b'L' => f = 0,
            _ => f = (f - 1).max(0),
        }
        fr[i] = f;
    }
    f = 0;
    for i in (0..n).rev() {
        match b[i] {
            b'L' => f = n as i32,
            b'R' => f = 0,
            _ => f = (f - 1).max(0),
        }
        fl[i] = f;
    }
    let mut res = String::with_capacity(n);
    for i in 0..n {
        if fr[i] > fl[i] {
            res.push('R');
        } else if fr[i] < fl[i] {
            res.push('L');
        } else {
            res.push('.');
        }
    }
    res
}

fn main() {
    println!("{}", push_dominoes(".L.R....L")); // LL.RRRLLL
    println!("{}", push_dominoes("..R...L.L")); // ..RR.LLLL
}
