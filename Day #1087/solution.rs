// B is a rotation of A iff lengths match and B is a substring of A+A. Time O(n), Space O(n).
fn is_rotation(a: &str, b: &str) -> bool {
    a.len() == b.len() && format!("{}{}", a, a).contains(b)
}

fn main() {
    println!("{}", is_rotation("abcde", "cdeab"));
    println!("{}", is_rotation("abc", "acb"));
}
