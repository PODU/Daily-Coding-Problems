// Region cutting by slashes: split each cell into 4 triangles, union-find.
// Time: O(N*M * alpha), Space: O(N*M).
fn find(par: &mut Vec<usize>, mut x: usize) -> usize {
    while par[x] != x {
        par[x] = par[par[x]];
        x = par[x];
    }
    x
}

fn uni(par: &mut Vec<usize>, a: usize, b: usize) {
    let ra = find(par, a);
    let rb = find(par, b);
    par[ra] = rb;
}

fn regions(g: &[&str]) -> usize {
    let rows: Vec<&[u8]> = g.iter().map(|s| s.as_bytes()).collect();
    let n = rows.len();
    let m = rows[0].len();
    let mut par: Vec<usize> = (0..n * m * 4).collect();
    for r in 0..n {
        for c in 0..m {
            let base = (r * m + c) * 4;
            let ch = rows[r][c];
            if ch == b'/' {
                uni(&mut par, base, base + 3);
                uni(&mut par, base + 1, base + 2);
            } else if ch == b'\\' {
                uni(&mut par, base, base + 1);
                uni(&mut par, base + 2, base + 3);
            } else {
                uni(&mut par, base, base + 1);
                uni(&mut par, base + 1, base + 2);
                uni(&mut par, base + 2, base + 3);
            }
            if c + 1 < m {
                uni(&mut par, base + 1, ((r * m + c + 1) * 4) + 3);
            }
            if r + 1 < n {
                uni(&mut par, base + 2, (((r + 1) * m + c) * 4));
            }
        }
    }
    let mut cnt = 0;
    for i in 0..n * m * 4 {
        if find(&mut par, i) == i {
            cnt += 1;
        }
    }
    cnt
}

fn main() {
    let g = vec![
        "\\    /",
        " \\  / ",
        "  \\/  ",
    ];
    println!("{}", regions(&g));
}
