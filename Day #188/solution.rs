// Day 188: Closure-in-a-loop late binding (Rust analog of the Python snippet).
// The "buggy" version shares one Rc<Cell<i32>> so every closure reads the final value 3.
// Fix: capture a copy of the value per iteration via `move`. Time/Space O(n).
use std::cell::Cell;
use std::rc::Rc;

fn make_functions_buggy() -> Vec<Box<dyn Fn() -> i32>> {
    let shared = Rc::new(Cell::new(0));
    let mut funcs: Vec<Box<dyn Fn() -> i32>> = Vec::new();
    for v in [1, 2, 3] {
        shared.set(v);
        let s = Rc::clone(&shared);
        funcs.push(Box::new(move || s.get())); // all read final shared value -> 3
    }
    funcs
}

fn make_functions_fixed() -> Vec<Box<dyn Fn() -> i32>> {
    let mut funcs: Vec<Box<dyn Fn() -> i32>> = Vec::new();
    for v in [1, 2, 3] {
        funcs.push(Box::new(move || v)); // snapshot per iteration
    }
    funcs
}

fn main() {
    println!("Late binding prints:");
    for f in make_functions_buggy() {
        println!("{}", f());
    }
    println!("Fixed prints:");
    for f in make_functions_fixed() {
        println!("{}", f());
    }
}
