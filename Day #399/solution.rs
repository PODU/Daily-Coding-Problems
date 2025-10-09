// Partition into 3 contiguous equal-sum parts: greedy prefix cut at target, absorbing trailing zeros. O(n) time, O(n) space.
fn partition3(l: &[i32]) -> Option<Vec<Vec<i32>>> {
    let total: i32 = l.iter().sum();
    if total % 3 != 0 {
        return None;
    }
    let target = total / 3;
    let mut res: Vec<Vec<i32>> = Vec::new();
    let mut cur: Vec<i32> = Vec::new();
    let mut running = 0;
    let n = l.len();
    for i in 0..n {
        cur.push(l[i]);
        running += l[i];
        // close part when sum hits target and next element is non-zero (zeros stay attached)
        if res.len() < 2 && running == target && (i + 1 == n || l[i + 1] != 0) {
            res.push(std::mem::take(&mut cur));
            running = 0;
        }
    }
    res.push(cur);
    if res.len() != 3 || res.iter().any(|p| p.iter().sum::<i32>() != target) {
        return None;
    }
    Some(res)
}

fn format(parts: &Option<Vec<Vec<i32>>>) -> String {
    match parts {
        None => "None".to_string(),
        Some(ps) => {
            let outer: Vec<String> = ps
                .iter()
                .map(|p| {
                    let nums: Vec<String> = p.iter().map(|x| x.to_string()).collect();
                    format!("[{}]", nums.join(", "))
                })
                .collect();
            format!("[{}]", outer.join(", "))
        }
    }
}

fn main() {
    let l = vec![3, 5, 8, 0, 8];
    println!("{}", format(&partition3(&l)));
}
