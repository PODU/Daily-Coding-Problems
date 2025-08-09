// Day 91: Closure-in-loop. Rust's `move` closures capture each value by copy, so
// the natural idiom already binds the current value per iteration. O(n).
fn main() {
    let mut fixed: Vec<Box<dyn Fn() -> i32>> = Vec::new();
    for i in 0..10 {
        fixed.push(Box::new(move || i)); // each closure owns its own i
    }
    let out: Vec<i32> = fixed.iter().map(|f| f()).collect();
    // The Python "broken" version prints 9 ten times (shared variable):
    println!("Broken (prints 9 ten times): {:?}", vec![9; 10]);
    println!("Fixed: {:?}", out);
}
