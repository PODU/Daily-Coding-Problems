// Selection sort using only reverse(lst,i,j). For each i, find min in [i..n-1],
// reverse [i..m] to bring it to front. Time O(n^2), Space O(1).

fn reverse_range(lst: &mut [i32], mut i: usize, mut j: usize) {
    while i < j {
        lst.swap(i, j);
        i += 1;
        j -= 1;
    }
}

fn sort_with_reverse(lst: &mut [i32]) {
    let n = lst.len();
    for i in 0..n {
        let mut m = i;
        for k in (i + 1)..n {
            if lst[k] < lst[m] {
                m = k;
            }
        }
        reverse_range(lst, i, m);
    }
}

fn main() {
    let mut data = vec![3, 1, 2, 5, 4];
    sort_with_reverse(&mut data);
    let parts: Vec<String> = data.iter().map(|v| v.to_string()).collect();
    println!("{}", parts.join(" "));
}
