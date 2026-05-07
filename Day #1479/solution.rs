// Day 1479: Partition a list into <x, ==x, >x around pivot x.
// Stable bucketing into three lists preserves relative order. Time O(N), Space O(N).

fn partition(lst: &[i32], x: i32) -> Vec<i32> {
    let (mut less, mut equal, mut greater) = (Vec::new(), Vec::new(), Vec::new());
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
    println!("{:?}", partition(&[9, 12, 3, 5, 14, 10, 10], 10));
    // [9, 3, 5, 10, 10, 12, 14]
}
