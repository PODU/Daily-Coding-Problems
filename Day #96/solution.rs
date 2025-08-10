// Day 96: All permutations via backtracking on the remaining elements, yielding
// lexicographic order. O(n*n!) time.
fn backtrack(path: &mut Vec<i32>, rem: &[i32], res: &mut Vec<Vec<i32>>) {
    if rem.is_empty() {
        res.push(path.clone());
        return;
    }
    for i in 0..rem.len() {
        path.push(rem[i]);
        let mut next = rem.to_vec();
        next.remove(i);
        backtrack(path, &next, res);
        path.pop();
    }
}

fn main() {
    let mut res = Vec::new();
    backtrack(&mut Vec::new(), &[1, 2, 3], &mut res);
    println!("{:?}", res);
    // [[1, 2, 3], [1, 3, 2], [2, 1, 3], [2, 3, 1], [3, 1, 2], [3, 2, 1]]
}
