// Longest strictly Increasing Subsequence length via patience sorting + binary search.
// Time O(N log N), Space O(N). Also reconstructs one valid LIS.

fn lis_length(nums: &[i32]) -> (usize, Vec<i32>) {
    let mut tails: Vec<i32> = Vec::new();       // value of smallest tail per length
    let mut tails_idx: Vec<usize> = Vec::new(); // index in nums of that tail
    let mut prev: Vec<i32> = vec![-1; nums.len()];
    for (i, &x) in nums.iter().enumerate() {
        // lower_bound over tails
        let mut lo = 0usize;
        let mut hi = tails.len();
        while lo < hi {
            let mid = (lo + hi) / 2;
            if tails[mid] < x {
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }
        let pos = lo;
        if pos == tails.len() {
            tails.push(x);
            tails_idx.push(i);
        } else {
            tails[pos] = x;
            tails_idx[pos] = i;
        }
        prev[i] = if pos > 0 { tails_idx[pos - 1] as i32 } else { -1 };
    }
    let mut seq: Vec<i32> = Vec::new();
    let mut k: i32 = if tails_idx.is_empty() {
        -1
    } else {
        tails_idx[tails_idx.len() - 1] as i32
    };
    while k != -1 {
        seq.push(nums[k as usize]);
        k = prev[k as usize];
    }
    seq.reverse();
    (tails.len(), seq)
}

fn main() {
    let nums = [0, 8, 4, 12, 2, 10, 6, 14, 1, 9, 5, 13, 3, 11, 7, 15];
    let (length, seq) = lis_length(&nums);
    println!("{}", length);
    println!("{:?}", seq);
}
