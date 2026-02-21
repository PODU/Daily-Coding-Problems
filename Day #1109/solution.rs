// Day 1109: Three-way (Dutch national flag) partition around pivot x.
// Bucket into <x, ==x, >x preserving relative order to match the example.
// Time: O(N). Space: O(N).
fn partition_three(lst: &[i32], x: i32) -> Vec<i32> {
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
    println!("{:?}", partition_three(&[9, 12, 3, 5, 14, 10, 10], 10)); // [9, 3, 5, 10, 10, 12, 14]
}
