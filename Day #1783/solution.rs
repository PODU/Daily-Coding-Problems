// Count regions split by '/','\\',' ' via Union-Find: each cell = 4 triangles (T,R,B,L).
// Union within cell per char and across neighbors. Time O(R*C*alpha), Space O(R*C).
use std::collections::HashSet;

struct Dsu {
    p: Vec<usize>,
}

impl Dsu {
    fn new(n: usize) -> Self {
        Dsu { p: (0..n).collect() }
    }
    fn find(&mut self, mut x: usize) -> usize {
        while self.p[x] != x {
            self.p[x] = self.p[self.p[x]];
            x = self.p[x];
        }
        x
    }
    fn union(&mut self, a: usize, b: usize) {
        let ra = self.find(a);
        let rb = self.find(b);
        self.p[ra] = rb;
    }
}

fn regions(grid: &[&str]) -> usize {
    let r_n = grid.len();
    let c_n = grid[0].chars().count();
    let rows: Vec<Vec<char>> = grid.iter().map(|s| s.chars().collect()).collect();
    let mut dsu = Dsu::new(4 * r_n * c_n);
    let idx = |r: usize, c: usize, t: usize| 4 * (r * c_n + c) + t;
    for r in 0..r_n {
        for c in 0..c_n {
            let ch = rows[r][c];
            match ch {
                ' ' => {
                    dsu.union(idx(r, c, 0), idx(r, c, 1));
                    dsu.union(idx(r, c, 1), idx(r, c, 2));
                    dsu.union(idx(r, c, 2), idx(r, c, 3));
                }
                '/' => {
                    dsu.union(idx(r, c, 0), idx(r, c, 3));
                    dsu.union(idx(r, c, 1), idx(r, c, 2));
                }
                _ => {
                    dsu.union(idx(r, c, 0), idx(r, c, 1));
                    dsu.union(idx(r, c, 2), idx(r, c, 3));
                }
            }
            if c + 1 < c_n {
                dsu.union(idx(r, c, 1), idx(r, c + 1, 3));
            }
            if r + 1 < r_n {
                dsu.union(idx(r, c, 2), idx(r + 1, c, 0));
            }
        }
    }
    let mut roots = HashSet::new();
    for i in 0..4 * r_n * c_n {
        roots.insert(dsu.find(i));
    }
    roots.len()
}

fn main() {
    let grid = ["\\    /", " \\  / ", "  \\/  "];
    println!("{}", regions(&grid)); // 3
}
