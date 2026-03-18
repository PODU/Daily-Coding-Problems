// Graph k-colorability via backtracking: assign colors 1..k to vertices in order,
// skipping conflicts. Time O(k^n) worst case, Space O(n).
fn safe(v: usize, g: &Vec<Vec<i32>>, color: &Vec<i32>, c: i32) -> bool {
    for i in 0..g.len() {
        if g[v][i] == 1 && color[i] == c {
            return false;
        }
    }
    true
}

fn colorize(v: usize, g: &Vec<Vec<i32>>, k: i32, color: &mut Vec<i32>) -> bool {
    if v == g.len() {
        return true;
    }
    for c in 1..=k {
        if safe(v, g, color, c) {
            color[v] = c;
            if colorize(v + 1, g, k, color) {
                return true;
            }
            color[v] = 0;
        }
    }
    false
}

fn is_k_colorable(g: &Vec<Vec<i32>>, k: i32) -> bool {
    let mut color = vec![0; g.len()];
    colorize(0, g, k, &mut color)
}

fn main() {
    let g = vec![vec![0, 1, 1], vec![1, 0, 1], vec![1, 1, 0]];
    println!("{}", is_k_colorable(&g, 2));
    println!("{}", is_k_colorable(&g, 3));
}
