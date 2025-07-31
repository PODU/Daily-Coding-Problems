// Day 56: Graph k-colorability via backtracking.
// Time: O(k^V) worst case, Space: O(V).
fn can_color(g: &[Vec<i32>], k: i32, color: &mut [i32], v: usize) -> bool {
    let n = g.len();
    if v == n {
        return true;
    }
    for c in 1..=k {
        let mut ok = true;
        for u in 0..n {
            if g[v][u] == 1 && color[u] == c {
                ok = false;
                break;
            }
        }
        if !ok {
            continue;
        }
        color[v] = c;
        if can_color(g, k, color, v + 1) {
            return true;
        }
        color[v] = 0;
    }
    false
}

fn k_colorable(g: &[Vec<i32>], k: i32) -> bool {
    let mut color = vec![0; g.len()];
    can_color(g, k, &mut color, 0)
}

fn main() {
    // Triangle graph: needs 3 colors.
    let g = vec![
        vec![0, 1, 1],
        vec![1, 0, 1],
        vec![1, 1, 0],
    ];
    println!("{}", k_colorable(&g, 2)); // false
    println!("{}", k_colorable(&g, 3)); // true
}
