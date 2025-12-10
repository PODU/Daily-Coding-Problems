// Day 730: Closure-in-a-loop late binding (Rust analogue).
// Capturing a shared Rc<RefCell> -> all closures observe its final value (3,3,3).
// Fix: `move` each per-iteration value into its own closure (1,2,3).
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    // Buggy: shared mutable cell captured by all closures
    let shared = Rc::new(RefCell::new(0));
    let mut buggy: Vec<Box<dyn Fn()>> = Vec::new();
    for v in [1, 2, 3] {
        *shared.borrow_mut() = v;
        let s = Rc::clone(&shared);
        buggy.push(Box::new(move || println!("{}", s.borrow())));
    }
    for f in &buggy {
        f(); // 3, 3, 3
    }

    // Fixed: move the value itself into each closure
    let mut fixed: Vec<Box<dyn Fn()>> = Vec::new();
    for v in [1, 2, 3] {
        fixed.push(Box::new(move || println!("{}", v)));
    }
    for f in &fixed {
        f(); // 1, 2, 3
    }
}
