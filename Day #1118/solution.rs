// Day 1118 - Late-binding closure pitfall
// Sharing one Cell makes every closure read its final value (9).
// Fix: move a per-iteration copy into each closure.
use std::cell::Cell;
use std::rc::Rc;

fn main() {
    let holder = Rc::new(Cell::new(0));
    let mut buggy: Vec<Box<dyn Fn() -> i32>> = Vec::new();
    let mut fixed: Vec<Box<dyn Fn() -> i32>> = Vec::new();
    for k in 0..10 {
        holder.set(k);
        let h = holder.clone();
        buggy.push(Box::new(move || h.get())); // shared state
        fixed.push(Box::new(move || k));       // own copy
    }

    println!("Buggy output (all 9):");
    for f in &buggy {
        println!("{}", f());
    }
    println!("Fixed output (0-9):");
    for f in &fixed {
        println!("{}", f());
    }
}
