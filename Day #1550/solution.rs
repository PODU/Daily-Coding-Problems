// Closure capture demo: "buggy" closures share one Rc<Cell> (prints 9 x10);
// "fixed" closures move a per-iteration value (prints 0..9). Time O(n), Space O(n).
use std::cell::Cell;
use std::rc::Rc;

fn main() {
    // Buggy: all closures share the SAME mutable cell, whose final value is 9.
    let shared = Rc::new(Cell::new(0));
    let mut buggy: Vec<Box<dyn Fn() -> i32>> = Vec::new();
    for i in 0..10 {
        shared.set(i);
        let s = Rc::clone(&shared);
        buggy.push(Box::new(move || s.get()));
    }
    for f in &buggy {
        println!("{}", f());
    }

    println!();

    // Fixed: each closure moves its own per-iteration value.
    let mut fixed: Vec<Box<dyn Fn() -> i32>> = Vec::new();
    for i in 0..10 {
        fixed.push(Box::new(move || i));
    }
    for f in &fixed {
        println!("{}", f());
    }
}
