// Kaprekar's routine: repeatedly subtract ascending-digit from descending-digit
// (4-digit, zero-padded) until 6174; count steps. Time: O(7) iters, Space: O(1).
fn kaprekar(mut n: u32) -> u32 {
    let mut steps = 0;
    while n != 6174 {
        let s = format!("{:04}", n);
        let mut chars: Vec<char> = s.chars().collect();
        chars.sort();
        let asc: u32 = chars.iter().collect::<String>().parse().unwrap();
        chars.sort_by(|a, b| b.cmp(a));
        let desc: u32 = chars.iter().collect::<String>().parse().unwrap();
        n = desc - asc;
        steps += 1;
    }
    steps
}

fn main() {
    println!("{}", kaprekar(1234));
}
