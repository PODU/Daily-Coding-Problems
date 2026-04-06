// Rearrange list values to low->high->low->high. One pass swapping adjacent
// values to enforce the alternating relation. Time O(n), Space O(1).
// (Uses a Vec to model the list cleanly in safe Rust.)

fn zigzag(a: &mut [i32]) {
    let mut low = true; // current pair should satisfy a <= b
    let mut i = 0;
    while i + 1 < a.len() {
        if (low && a[i] > a[i + 1]) || (!low && a[i] < a[i + 1]) {
            a.swap(i, i + 1);
        }
        i += 1;
        low = !low;
    }
}

fn main() {
    let mut list = vec![1, 2, 3, 4, 5];
    zigzag(&mut list);
    let s: Vec<String> = list.iter().map(|v| v.to_string()).collect();
    println!("{}", s.join(" -> ")); // 1 -> 3 -> 2 -> 5 -> 4
}
