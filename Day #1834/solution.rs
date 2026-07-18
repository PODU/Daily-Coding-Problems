// Min swaps to seat couples together. Union couples in each seat-pair; answer is
// N - (#connected components). O(N * alpha(N)).
fn find(par: &mut Vec<usize>, x: usize) -> usize {
    let mut x = x;
    while par[x] != x {
        par[x] = par[par[x]];
        x = par[x];
    }
    x
}

fn min_swaps(row: &[usize]) -> usize {
    let n = row.len() / 2; // number of couples
    let mut par: Vec<usize> = (0..n).collect();
    let mut comps = n;
    for i in 0..n {
        let ra = find(&mut par, row[2 * i] / 2);
        let rb = find(&mut par, row[2 * i + 1] / 2);
        if ra != rb {
            par[ra] = rb;
            comps -= 1;
        }
    }
    n - comps
}

fn main() {
    let row = [0, 2, 1, 3]; // couples are (0,1) and (2,3)
    println!("{}", min_swaps(&row)); // 1
}
