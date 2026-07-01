// Left fold (reduce): single linear pass applying combiner to accumulator. O(n) time, O(1) extra space.

fn reduce<T: Copy, A>(arr: &[T], combining_fn: impl Fn(A, T) -> A, initial_value: A) -> A {
    let mut acc = initial_value;
    for &x in arr {
        acc = combining_fn(acc, x);
    }
    acc
}

fn sum(arr: &[i64]) -> i64 {
    reduce(arr, |acc, x| acc + x, 0)
}

fn main() {
    println!("{}", sum(&[1, 2, 3, 4, 5]));
    println!("{}", reduce(&[1, 2, 3, 4], |acc, x| acc * x, 1));
}
