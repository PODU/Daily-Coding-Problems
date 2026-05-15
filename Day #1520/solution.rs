// Single number among triples: bitwise ones/twos accumulators isolate the unique value.
// Time: O(n). Space: O(1).

fn single_number(nums: &[i32]) -> i32 {
    let mut ones: i32 = 0;
    let mut twos: i32 = 0;
    for &x in nums {
        ones = (ones ^ x) & !twos;
        twos = (twos ^ x) & !ones;
    }
    ones // i32 handles negatives via two's complement
}

fn main() {
    let a = [6, 1, 3, 3, 3, 6, 6];
    let b = [13, 19, 13, 13];
    println!("{}", single_number(&a)); // 1
    println!("{}", single_number(&b)); // 19
}
