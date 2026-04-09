// Day 1326: Implement reduce/fold — fold an array left to right with a combining function and an initial value.
// O(n) calls to the combiner, O(1) extra space.

fn reduce<T, A, F: Fn(A, &T) -> A>(lst: &[T], combine: F, init: A) -> A {
    let mut acc = init;
    for x in lst {
        acc = combine(acc, x);
    }
    acc
}

fn main() {
    let lst = vec![1, 2, 3, 4, 5];
    let total = reduce(&lst, |a, &b| a + b, 0);
    println!("{}", total); // 15
}
