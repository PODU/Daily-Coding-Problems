// reduce/fold: apply combiner left-to-right starting from init. Time O(n), Space O(1).
fn reduce<T, A, F>(lst: &[T], combine: F, init: A) -> A
where
    F: Fn(A, &T) -> A,
{
    let mut acc = init;
    for x in lst {
        acc = combine(acc, x);
    }
    acc
}

fn main() {
    let lst = vec![1, 2, 3, 4, 5];
    let s = reduce(&lst, |a, b| a + b, 0);
    println!("{}", s); // 15
}
