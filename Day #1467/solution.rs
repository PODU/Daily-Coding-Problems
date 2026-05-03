// All permutations via backtracking, picking remaining elements left-to-right (lexicographic order).
// Time O(n! * n), Space O(n) recursion + output.
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

fn permute(nums: &[i32]) -> Vec<Vec<i32>> {
    let mut res = Vec::new();
    let mut used = vec![false; nums.len()];
    let mut cur = Vec::new();
    backtrack(nums, &mut used, &mut cur, &mut res);
    res
}

fn main() {
    let res = permute(&[1, 2, 3]);
    let parts: Vec<String> = res
        .iter()
        .map(|p| {
            let nums: Vec<String> = p.iter().map(|x| x.to_string()).collect();
            format!("[{}]", nums.join(","))
        })
        .collect();
    println!("[{}]", parts.join(","));
}
