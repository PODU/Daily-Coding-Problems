// Custom left fold (reduce). acc=init; for each x: acc=f(acc,x). O(n) time, O(1) space.

fn my_reduce<T, A, F>(arr: &[T], f: F, init: A) -> A
where
    F: Fn(A, &T) -> A,
{
    let mut acc = init;
    for x in arr {
        acc = f(acc, x);
    }
    acc
}

fn add(a: i64, b: i64) -> i64 {
    a + b
}

fn sum(lst: &[i64]) -> i64 {
    my_reduce(lst, |a, x| add(a, *x), 0)
}

fn main() {
    let lst = [1, 2, 3, 4, 5];
    println!("{}", sum(&lst));
}
