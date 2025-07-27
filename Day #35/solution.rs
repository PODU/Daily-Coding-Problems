// Dutch National Flag 3-way partition (R<G<B). In-place, O(n) time, O(1) space, swaps only.
fn sort_rgb(a: &mut Vec<char>) {
    let mut low = 0usize;
    let mut mid = 0usize;
    let mut high = a.len() - 1;
    while mid <= high {
        match a[mid] {
            'R' => {
                a.swap(low, mid);
                low += 1;
                mid += 1;
            }
            'G' => {
                mid += 1;
            }
            _ => {
                a.swap(mid, high);
                if high == 0 {
                    break;
                }
                high -= 1;
            }
        }
    }
}

fn main() {
    let mut a = vec!['G', 'B', 'R', 'R', 'B', 'R', 'G'];
    sort_rgb(&mut a);
    let s: Vec<String> = a.iter().map(|c| format!("'{}'", c)).collect();
    println!("[{}]", s.join(", "));
}
