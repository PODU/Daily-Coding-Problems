// Day 1624: Steps of Kaprekar's routine to reach 6174.
// Iterate sort-desc minus sort-asc until 6174. Time O(1) (bounded ~7 iters).
fn kaprekar_steps(mut n: i32) -> i32 {
    let mut steps = 0;
    while n != 6174 {
        let s = format!("{:04}", n);
        let mut digits: Vec<char> = s.chars().collect();
        digits.sort();
        let asc: i32 = digits.iter().collect::<String>().parse().unwrap();
        digits.reverse();
        let desc: i32 = digits.iter().collect::<String>().parse().unwrap();
        n = desc - asc;
        steps += 1;
        if n == 0 {
            break;
        }
    }
    steps
}

fn main() {
    println!("{}", kaprekar_steps(1234));
}
