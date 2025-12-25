// Day 800: Rearrange list values into low->high->low... (wiggle).
// One pass over values: even idx a<=next, odd idx a>=next; swap if not.
// Time O(N), Space O(1). (Uses a Vec to model the list cleanly in safe Rust.)

fn wiggle(a: &mut [i32]) {
    let mut less = true;
    for i in 0..a.len().saturating_sub(1) {
        let bad = if less { a[i] > a[i + 1] } else { a[i] < a[i + 1] };
        if bad {
            a.swap(i, i + 1);
        }
        less = !less;
    }
}

fn main() {
    let mut list = vec![1, 2, 3, 4, 5];
    wiggle(&mut list);
    let parts: Vec<String> = list.iter().map(|v| v.to_string()).collect();
    println!("{}", parts.join(" -> ")); // 1 -> 3 -> 2 -> 5 -> 4
}
