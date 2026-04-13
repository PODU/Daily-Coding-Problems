// Longest run of consecutive 1s in binary. Bit trick: n &= (n<<1) collapses runs; iterations = longest run. O(run) time, O(1) space.

fn longest_run(mut n: u32) -> i32 {
    let mut count = 0;
    while n != 0 {
        n &= n << 1;
        count += 1;
    }
    count
}

fn main() {
    println!("{}", longest_run(156));
}
