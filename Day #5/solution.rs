// Closure-based pair: cons stores (a,b) in a closure; car/cdr apply a selector.
// Time: O(1), Space: O(1) per pair.
use std::rc::Rc;

type Pair = Rc<dyn Fn(&dyn Fn(i32, i32) -> i32) -> i32>;

fn cons(a: i32, b: i32) -> Pair {
    Rc::new(move |f: &dyn Fn(i32, i32) -> i32| f(a, b))
}
fn car(p: &Pair) -> i32 {
    p(&|a, _b| a)
}
fn cdr(p: &Pair) -> i32 {
    p(&|_a, b| b)
}

fn main() {
    println!("{}", car(&cons(3, 4)));
    println!("{}", cdr(&cons(3, 4)));
}
