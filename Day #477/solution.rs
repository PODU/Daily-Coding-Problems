// Rust closures capture by value with `move`, so the late-binding bug normally can't happen.
// Buggy analogue: all closures share one Rc<RefCell<i32>> and read its final value (9).
// Fix: each closure `move`s its own copy of i -> prints 0..9.
// (Python analogue: lambda: i prints 9 ten times; fix is lambda i=i: i.)
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let shared = Rc::new(RefCell::new(0));
    let mut buggy: Vec<Box<dyn Fn() -> i32>> = Vec::new();
    for i in 0..10 {
        *shared.borrow_mut() = i;
        let s = Rc::clone(&shared);
        buggy.push(Box::new(move || *s.borrow())); // all share the same cell
    }
    println!("Buggy:");
    for f in &buggy {
        println!("{}", f());
    }

    let mut fixed: Vec<Box<dyn Fn() -> i32>> = Vec::new();
    for i in 0..10 {
        fixed.push(Box::new(move || i)); // each closure owns its own copy
    }
    println!("Fixed:");
    for f in &fixed {
        println!("{}", f());
    }
}
