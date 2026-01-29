// Longest consecutive run of 1-bits via the bit trick: n &= (n>>1) shrinks every
// run by one each step; iterations until n==0 equals the longest run length.
// Time: O(longest run), Space: O(1).

fn longest_run(mut n: u32) -> u32 {
    let mut count = 0;
    while n != 0 {
        count += 1;
        n &= n >> 1;
    }
    count
}

fn main() {
    println!("{}", longest_run(156)); // 156 = 10011100 -> 3
}
