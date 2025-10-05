// Day 374: Lowest index i with arr[i]==i in a sorted, distinct-int array.
// f(i)=arr[i]-i is non-decreasing, so binary-search the leftmost i with
// f(i)>=0 and check equality. Time O(log n), Space O(1).

fn fixed_point(arr: &[i64]) -> Option<usize> {
    let (mut lo, mut hi) = (0i64, arr.len() as i64 - 1);
    let mut ans: i64 = -1;
    while lo <= hi {
        let mid = lo + (hi - lo) / 2;
        if arr[mid as usize] >= mid {
            ans = mid;
            hi = mid - 1;
        } else {
            lo = mid + 1;
        }
    }
    if ans != -1 && arr[ans as usize] == ans {
        Some(ans as usize)
    } else {
        None
    }
}

fn main() {
    let arr = [-5, -3, 2, 3];
    match fixed_point(&arr) {
        Some(i) => println!("{}", i), // 2
        None => println!("null"),
    }
}
