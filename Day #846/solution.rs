// Day 846: implement car/cdr for closure-based cons.
// cons returns a closure taking a selector; car/cdr pass a selector. O(1).
use std::rc::Rc;

type Pair = Rc<dyn Fn(fn(i32, i32) -> i32) -> i32>;

fn cons(a: i32, b: i32) -> Pair {
    Rc::new(move |f: fn(i32, i32) -> i32| f(a, b))
}
fn car(p: &Pair) -> i32 {
    p(|a, _b| a)
}
fn cdr(p: &Pair) -> i32 {
    p(|_a, b| b)
}

fn main() {
    println!("{}", car(&cons(3, 4))); // 3
    println!("{}", cdr(&cons(3, 4))); // 4
}
