// Closure capture demo: buggy closures share one Rc<RefCell> cell (final value);
// fix moves a per-iteration copy into each closure. O(n) time/space.
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    println!("Late binding (buggy):");
    let shared = Rc::new(RefCell::new(0));
    let mut buggy: Vec<Box<dyn Fn() -> i32>> = Vec::new();
    for x in [1, 2, 3] {
        *shared.borrow_mut() = x;
        let s = Rc::clone(&shared);
        buggy.push(Box::new(move || *s.borrow()));
    }
    for f in &buggy {
        println!("{}", f());
    }
    println!("Fixed (capture value):");
    let mut fixed: Vec<Box<dyn Fn() -> i32>> = Vec::new();
    for x in [1, 2, 3] {
        fixed.push(Box::new(move || x));
    }
    for f in &fixed {
        println!("{}", f());
    }
}
