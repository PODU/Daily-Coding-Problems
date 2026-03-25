// Day 1262: Closure-in-a-loop (late binding) demonstrated in Rust.
// Rust closures capture by move/borrow with ownership rules, so the Python-style bug
// (shared mutable late binding) must be modeled explicitly with Rc<RefCell<>>.
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    // Buggy analogue: all closures capture the same shared cell.
    let shared = Rc::new(RefCell::new(0));
    let mut buggy: Vec<Box<dyn Fn()>> = Vec::new();
    for i in 1..=3 {
        *shared.borrow_mut() = i;
        let s = Rc::clone(&shared);
        buggy.push(Box::new(move || println!("{}", s.borrow())));
    }
    // Fixed: each closure moves its own copy of i.
    let mut fixed: Vec<Box<dyn Fn()>> = Vec::new();
    for i in 1..=3 {
        fixed.push(Box::new(move || println!("{}", i)));
    }
    println!("Buggy output:");
    for f in &buggy {
        f();
    }
    println!("Fixed output:");
    for f in &fixed {
        f();
    }
}
