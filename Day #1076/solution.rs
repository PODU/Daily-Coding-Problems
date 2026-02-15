// Variadic alternating add/subtract via slice: first arg seeds the total,
// then the rest alternate +, -, +, ... O(n) time, O(1) space.
fn add_subtract(args: &[i64]) -> i64 {
    if args.is_empty() {
        return 0;
    }
    let mut result = args[0];
    let mut sign = 1i64;
    for &v in &args[1..] {
        result += sign * v;
        sign = -sign;
    }
    result
}

fn main() {
    println!("add_subtract(7) = {}",            add_subtract(&[7]));
    println!("add_subtract(1)(2)(3) = {}",      add_subtract(&[1, 2, 3]));
    println!("add_subtract(-5)(10)(3)(9) = {}", add_subtract(&[-5, 10, 3, 9]));
}
