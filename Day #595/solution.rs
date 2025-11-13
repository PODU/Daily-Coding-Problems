// Kaprekar's routine: repeatedly subtract ascending-digit number from
// descending-digit number (4-digit, zero-padded) until reaching 6174.
// Bounded to <=7 steps. Time O(1), Space O(1).

fn main() {
    let mut n: u32 = 1234;
    let mut steps = 0;
    while n != 6174 {
        let s = format!("{:04}", n);
        let mut chars: Vec<char> = s.chars().collect();
        chars.sort();
        let asc: u32 = chars.iter().collect::<String>().parse().unwrap();
        let desc: u32 = chars.iter().rev().collect::<String>().parse().unwrap();
        n = desc - asc;
        steps += 1;
        println!("{:04} - {:04} = {:04}", desc, asc, n);
    }
    println!("Steps: {}", steps);
}
