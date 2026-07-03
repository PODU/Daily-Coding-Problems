// Day 1758: Dutch national flag — segregate R, G, B in-place.
// Three pointers (low/mid/high), one pass. O(n) time, O(1) space.

fn sort_rgb(a: &mut Vec<char>) {
    let mut low = 0usize;
    let mut mid = 0usize;
    let mut high = a.len() as isize - 1;
    while mid as isize <= high {
        match a[mid] {
            'R' => {
                a.swap(low, mid);
                low += 1;
                mid += 1;
            }
            'G' => mid += 1,
            _ => {
                a.swap(mid, high as usize);
                high -= 1;
            }
        }
    }
}

fn main() {
    let mut a = vec!['G', 'B', 'R', 'R', 'B', 'R', 'G'];
    sort_rgb(&mut a);
    let parts: Vec<String> = a.iter().map(|c| format!("'{}'", c)).collect();
    println!("[{}]", parts.join(", "));
}
