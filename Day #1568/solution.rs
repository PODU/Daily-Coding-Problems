// Longest consecutive run of 1s using n &= (n<<1) bit trick. Time O(run length), space O(1).
fn longest_run(mut n: u64) -> i32 {
    let mut count = 0;
    while n != 0 {
        n &= n << 1;
        count += 1;
    }
    count
}

fn main() {
    println!("{}", longest_run(156)); // 10011100 -> 3
}
