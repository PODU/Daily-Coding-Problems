// Day 372: Count digits of a natural number without loops.
// Recursion: 1 digit for n<10, else 1 + digits(n/10). Time O(d), Space O(d).

fn num_digits(n: u64) -> u32 {
    if n < 10 {
        1
    } else {
        1 + num_digits(n / 10)
    }
}

fn main() {
    println!("{}", num_digits(0));      // 1
    println!("{}", num_digits(7));      // 1
    println!("{}", num_digits(42));     // 2
    println!("{}", num_digits(12345));  // 5
}
