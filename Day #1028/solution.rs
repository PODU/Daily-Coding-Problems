// Day 1028: Kaprekar's routine. Repeatedly subtract ascending- from descending-
// digit arrangement until 6174; count steps. Time O(steps), Space O(1).
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
    }
    steps
}

fn main() {
    println!("{}", kaprekar_steps(1234));
}
