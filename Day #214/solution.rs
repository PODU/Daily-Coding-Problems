// Day 214: Longest consecutive run of 1s in binary representation.
// Approach: n &= (n<<1) collapses runs; count iterations. Time O(longest run), Space O(1).
fn longest_run(mut n: u64) -> u32 {
    let mut count = 0;
    while n != 0 {
        n &= n << 1;
        count += 1;
    }
    count
}

fn main() {
    println!("{}", longest_run(156)); // 156 = 10011100 -> 3
}
