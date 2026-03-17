// Closure late-binding: closures sharing one Rc<Cell> all read its final value (9);
// fix = move closures each copying their own i. O(n) build/call, O(n) space.
use std::cell::Cell;
use std::rc::Rc;

fn main() {
    // Buggy: all closures share one cell, which ends at 9.
    let shared = Rc::new(Cell::new(0));
    let mut buggy: Vec<Box<dyn Fn() -> i32>> = Vec::new();
    for t in 0..10 {
        shared.set(t);
        let s = Rc::clone(&shared);
        buggy.push(Box::new(move || s.get()));
    }
    for f in &buggy {
        println!("{}", f());
    }

    println!("---");

    // Fixed: each move closure copies its own i.
    let mut fixed: Vec<Box<dyn Fn() -> i32>> = Vec::new();
    for i in 0..10 {
        fixed.push(Box::new(move || i));
    }
    for f in &fixed {
        println!("{}", f());
    }
}
