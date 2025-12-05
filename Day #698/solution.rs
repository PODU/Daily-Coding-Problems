// Day 698: Count regions a grid of '/'/'\\'/' ' is divided into.
// Approach: split each cell into 4 triangles (top,right,bottom,left) and union by
// the slash type plus across neighbors (Union-Find). Time O(R*C*a), Space O(R*C).
struct DSU {
    p: Vec<usize>,
}

impl DSU {
    fn new(n: usize) -> Self {
        DSU { p: (0..n).collect() }
    }
    fn find(&mut self, x: usize) -> usize {
        let mut x = x;
        while self.p[x] != x {
            self.p[x] = self.p[self.p[x]];
            x = self.p[x];
        }
        x
    }
    fn uni(&mut self, a: usize, b: usize) {
        let ra = self.find(a);
        let rb = self.find(b);
        self.p[ra] = rb;
    }
}

fn regions(grid: &[&str]) -> usize {
    let r = grid.len();
    let c = grid.iter().map(|s| s.chars().count()).max().unwrap_or(0);
    let mut g = vec![vec![' '; c]; r];
    for (i, s) in grid.iter().enumerate() {
        for (j, ch) in s.chars().enumerate() {
            g[i][j] = ch;
        }
    }
    let mut dsu = DSU::new(r * c * 4);
    for i in 0..r {
        for j in 0..c {
            let base = (i * c + j) * 4; // 0=T,1=R,2=B,3=L
            match g[i][j] {
                '/' => {
                    dsu.uni(base, base + 3);
                    dsu.uni(base + 1, base + 2);
                }
                '\\' => {
                    dsu.uni(base, base + 1);
                    dsu.uni(base + 2, base + 3);
                }
                _ => {
                    dsu.uni(base, base + 1);
                    dsu.uni(base + 1, base + 2);
                    dsu.uni(base + 2, base + 3);
                }
            }
            if j + 1 < c {
                dsu.uni(base + 1, (i * c + j + 1) * 4 + 3);
            }
            if i + 1 < r {
                dsu.uni(base + 2, ((i + 1) * c + j) * 4);
            }
        }
    }
    (0..r * c * 4).filter(|&i| dsu.find(i) == i).count()
}

fn main() {
    let grid = ["\\    /", " \\  /", "  \\/"];
    println!("{}", regions(&grid)); // 3
}
