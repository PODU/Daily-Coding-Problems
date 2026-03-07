// Day 1173: Build a min-heap Cartesian tree whose in-order traversal is S.
// Monotonic-stack construction using index links in arena vectors. Time O(N), Space O(N).

fn cartesian_tree(s: &[i32]) -> (Vec<i32>, Vec<i32>, Vec<i32>, i32) {
    let n = s.len();
    let val: Vec<i32> = s.to_vec();
    let mut left = vec![-1i32; n];
    let mut right = vec![-1i32; n];
    let mut st: Vec<usize> = Vec::new();
    for i in 0..n {
        let mut last: i32 = -1;
        while let Some(&top) = st.last() {
            if val[top] > val[i] {
                last = top as i32;
                st.pop();
            } else {
                break;
            }
        }
        left[i] = last;
        if let Some(&top) = st.last() {
            right[top] = i as i32;
        }
        st.push(i);
    }
    let root = if st.is_empty() { -1 } else { st[0] as i32 };
    (val, left, right, root)
}

fn inorder(i: i32, left: &[i32], right: &[i32], val: &[i32], out: &mut Vec<i32>) {
    if i < 0 {
        return;
    }
    let u = i as usize;
    inorder(left[u], left, right, val, out);
    out.push(val[u]);
    inorder(right[u], left, right, val, out);
}

fn main() {
    let s = [3, 2, 6, 1, 9];
    let (val, left, right, root) = cartesian_tree(&s);
    let mut chk = Vec::new();
    inorder(root, &left, &right, &val, &mut chk);
    assert_eq!(chk, s.to_vec()); // verifies in-order == S
    println!("      1");
    println!("    /   \\");
    println!("  2       9");
    println!(" / \\");
    println!("3   6");
}
