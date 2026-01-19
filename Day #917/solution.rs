// Partition linked list: stable split into <k and >=k lists, then concatenate.
// Implemented over a Vec of (val) to keep ownership simple; preserves stable order.
// Time O(n), Space O(n) for output vector.
fn partition(vals: &[i32], k: i32) -> Vec<i32> {
    let mut less: Vec<i32> = Vec::new();
    let mut ge: Vec<i32> = Vec::new();
    for &v in vals {
        if v < k {
            less.push(v);
        } else {
            ge.push(v);
        }
    }
    less.extend(ge);
    less
}

fn main() {
    let vals = [5, 1, 8, 0, 3];
    let result = partition(&vals, 3);
    let parts: Vec<String> = result.iter().map(|x| x.to_string()).collect();
    println!("{}", parts.join(" -> "));
}
