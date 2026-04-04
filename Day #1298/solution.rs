// Day 1298: Count Kaprekar steps to reach 6174.
// Repeatedly sort digits desc - asc until 6174. Converges in <=7 steps. O(steps) time.
fn kaprekar_steps(mut n: u32) -> u32 {
    let mut steps = 0;
    while n != 6174 {
        let s = format!("{:04}", n);
        let mut digits: Vec<char> = s.chars().collect();
        digits.sort();
        let asc: u32 = digits.iter().collect::<String>().parse().unwrap();
        digits.reverse();
        let desc: u32 = digits.iter().collect::<String>().parse().unwrap();
        n = desc - asc;
        steps += 1;
        if n == 0 {
            break;
        }
    }
    steps
}

fn main() {
    println!("{}", kaprekar_steps(1234)); // 3
}
