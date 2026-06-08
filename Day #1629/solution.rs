// Three-way stable partition around pivot x: collect <x, ==x, >x in order, concat.
// Time O(n), Space O(n).
fn partition3(x: i32, lst: &[i32]) -> Vec<i32> {
    let mut less = Vec::new();
    let mut equal = Vec::new();
    let mut greater = Vec::new();
    for &v in lst {
        if v < x {
            less.push(v);
        } else if v == x {
            equal.push(v);
        } else {
            greater.push(v);
        }
    }
    less.extend(equal);
    less.extend(greater);
    less
}

fn main() {
    let res = partition3(10, &[9, 12, 3, 5, 14, 10, 10]);
    let parts: Vec<String> = res.iter().map(|v| v.to_string()).collect();
    println!("[{}]", parts.join(", "));
}
