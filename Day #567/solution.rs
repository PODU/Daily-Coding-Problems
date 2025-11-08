// Day 567: Implement car/cdr from closure-based cons.
// cons(a,b) returns a closure taking selector f -> f(a,b). Time O(1), Space O(1).

// A pair is a closure that applies a selector to the two stored values.
fn cons(a: i32, b: i32) -> impl Fn(&dyn Fn(i32, i32) -> i32) -> i32 {
    move |f: &dyn Fn(i32, i32) -> i32| f(a, b)
}

fn car<P: Fn(&dyn Fn(i32, i32) -> i32) -> i32>(pair: &P) -> i32 {
    pair(&|a, _b| a)
}

fn cdr<P: Fn(&dyn Fn(i32, i32) -> i32) -> i32>(pair: &P) -> i32 {
    pair(&|_a, b| b)
}

fn main() {
    let p = cons(3, 4);
    println!("{}", car(&p));
    println!("{}", cdr(&p));
}
