// cons returns a closure pair(f)=f(a,b); car/cdr apply a selector. O(1).
fn cons(a: i32, b: i32) -> impl Fn(fn(i32, i32) -> i32) -> i32 {
    move |f| f(a, b)
}

fn car<P: Fn(fn(i32, i32) -> i32) -> i32>(p: P) -> i32 {
    p(|a, _b| a)
}

fn cdr<P: Fn(fn(i32, i32) -> i32) -> i32>(p: P) -> i32 {
    p(|_a, b| b)
}

fn main() {
    println!("{}", car(cons(3, 4)));
    println!("{}", cdr(cons(3, 4)));
}
