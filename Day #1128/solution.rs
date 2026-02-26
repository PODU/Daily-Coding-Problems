// All permutations of a list of digits in lexicographic order.
// Backtracking over sorted digits. O(n!*n) time, O(n) extra space.
fn backtrack(digits: &Vec<i32>, used: &mut Vec<bool>, cur: &mut Vec<i32>, res: &mut Vec<Vec<i32>>) {
    if cur.len() == digits.len() {
        res.push(cur.clone());
        return;
    }
    for i in 0..digits.len() {
        if used[i] {
            continue;
        }
        used[i] = true;
        cur.push(digits[i]);
        backtrack(digits, used, cur, res);
        cur.pop();
        used[i] = false;
    }
}

fn main() {
    let mut digits = vec![1, 2, 3];
    digits.sort();
    let mut used = vec![false; digits.len()];
    let mut cur = Vec::new();
    let mut res = Vec::new();
    backtrack(&digits, &mut used, &mut cur, &mut res);

    let inner: Vec<String> = res
        .iter()
        .map(|p| {
            let parts: Vec<String> = p.iter().map(|x| x.to_string()).collect();
            format!("[{}]", parts.join(","))
        })
        .collect();
    println!("[{}]", inner.join(","));
}
