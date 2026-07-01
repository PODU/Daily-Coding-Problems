// Day 1747: All permutations of a list of digits.
// Approach: backtracking with a used[] mask, iterating values in order -> lexicographic.
// Time O(n * n!), space O(n) recursion (plus O(n!) for the output).

fn backtrack(nums: &[i32], used: &mut Vec<bool>, cur: &mut Vec<i32>, res: &mut Vec<Vec<i32>>) {
    if cur.len() == nums.len() {
        res.push(cur.clone());
        return;
    }
    for i in 0..nums.len() {
        if used[i] {
            continue;
        }
        used[i] = true;
        cur.push(nums[i]);
        backtrack(nums, used, cur, res);
        cur.pop();
        used[i] = false;
    }
}

fn permutations(nums: &[i32]) -> Vec<Vec<i32>> {
    let mut res = Vec::new();
    let mut used = vec![false; nums.len()];
    let mut cur = Vec::new();
    backtrack(nums, &mut used, &mut cur, &mut res);
    res
}

fn main() {
    let nums = vec![1, 2, 3];
    let res = permutations(&nums);
    let inner: Vec<String> = res
        .iter()
        .map(|p| {
            let s: Vec<String> = p.iter().map(|v| v.to_string()).collect();
            format!("[{}]", s.join(","))
        })
        .collect();
    println!("[{}]", inner.join(","));
}
