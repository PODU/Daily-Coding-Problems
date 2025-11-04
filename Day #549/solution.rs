// Single number among triples: sum each bit position mod 3 to rebuild the unique value. O(N) time, O(1) space.

fn single_number(nums: &[i32]) -> i32 {
    let mut result = 0i32;
    for b in 0..32 {
        let mut cnt = 0;
        for &x in nums {
            cnt += (x >> b) & 1;
        }
        if cnt % 3 != 0 {
            result |= 1 << b;
        }
    }
    result
}

fn main() {
    println!("{}", single_number(&[6, 1, 3, 3, 3, 6, 6]));
    println!("{}", single_number(&[13, 19, 13, 13]));
}
