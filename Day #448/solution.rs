// Day 448: Dutch National Flag sort of R/G/B. O(n) time, O(1) space, in-place
// with three pointers (low=R boundary, high=B boundary, mid=scanner).

fn sort_rgb(a: &mut Vec<char>) {
    let mut low = 0usize;
    let mut mid = 0usize;
    let mut high = a.len();
    while mid < high {
        match a[mid] {
            'R' => {
                a.swap(low, mid);
                low += 1;
                mid += 1;
            }
            'G' => mid += 1,
            _ => {
                high -= 1;
                a.swap(mid, high);
            }
        }
    }
}

fn main() {
    let mut a = vec!['G', 'B', 'R', 'R', 'B', 'R', 'G'];
    sort_rgb(&mut a);
    println!("{:?}", a);
    // ['R', 'R', 'R', 'G', 'G', 'B', 'B']
}
