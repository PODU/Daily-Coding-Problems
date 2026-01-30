// Day 991: Closure late-binding demonstration.
// Buggy closures share one Rc<Cell> (all read 3); fixed closures capture a
// per-iteration value copy (1,2,3). O(n) time/space.
use std::cell::Cell;
use std::rc::Rc;

fn make_buggy() -> Vec<Box<dyn Fn() -> i32>> {
    let mut v: Vec<Box<dyn Fn() -> i32>> = Vec::new();
    let shared = Rc::new(Cell::new(0));
    for val in [1, 2, 3] {
        shared.set(val); // single shared cell
        let s = Rc::clone(&shared);
        v.push(Box::new(move || s.get()));
    }
    v
}

fn make_fixed() -> Vec<Box<dyn Fn() -> i32>> {
    let mut v: Vec<Box<dyn Fn() -> i32>> = Vec::new();
    for val in [1, 2, 3] {
        v.push(Box::new(move || val)); // capture value copy
    }
    v
}

fn main() {
    print!("Buggy:");
    for f in make_buggy() {
        print!(" {}", f());
    }
    print!("\nFixed:");
    for f in make_fixed() {
        print!(" {}", f());
    }
    println!();
}
