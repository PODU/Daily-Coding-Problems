// cons returns a closure taking a selector(a,b)->int; car/cdr pass selectors
// returning first/second arg. All O(1).

type Pair = Box<dyn Fn(&dyn Fn(i32, i32) -> i32) -> i32>;

fn cons(a: i32, b: i32) -> Pair {
    Box::new(move |f: &dyn Fn(i32, i32) -> i32| f(a, b))
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
