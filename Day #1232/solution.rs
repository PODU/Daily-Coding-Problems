// Count regions carved by slashes via 4-triangle split + Union-Find.
// Time O(n*m a(n*m)), Space O(n*m).
struct Dsu {
    p: Vec<usize>,
    count: usize,
}

impl Dsu {
    fn new(n: usize) -> Self {
        Dsu { p: (0..n).collect(), count: n }
    }
    fn find(&mut self, mut x: usize) -> usize {
        while self.p[x] != x {
            self.p[x] = self.p[self.p[x]];
            x = self.p[x];
        }
        x
    }
    fn unite(&mut self, a: usize, b: usize) {
        let (ra, rb) = (self.find(a), self.find(b));
        if ra != rb {
            self.p[ra] = rb;
            self.count -= 1;
        }
    }
}

fn regions(grid: &[&str]) -> usize {
    let rows = grid.len();
    let cols = if rows > 0 { grid[0].chars().count() } else { 0 };
    let mut dsu = Dsu::new(4 * rows * cols);
    let idx = |r: usize, c: usize, k: usize| 4 * (r * cols + c) + k;
    for r in 0..rows {
        let chars: Vec<char> = grid[r].chars().collect();
        for c in 0..cols {
            let ch = chars[c];
            let (t, ri, b, le) = (idx(r, c, 0), idx(r, c, 1), idx(r, c, 2), idx(r, c, 3));
            match ch {
                '/' => { dsu.unite(t, le); dsu.unite(ri, b); }
                '\\' => { dsu.unite(t, ri); dsu.unite(le, b); }
                _ => { dsu.unite(t, ri); dsu.unite(ri, b); dsu.unite(b, le); }
            }
            if c + 1 < cols { dsu.unite(ri, idx(r, c + 1, 3)); }
            if r + 1 < rows { dsu.unite(b, idx(r + 1, c, 0)); }
        }
    }
    dsu.count
}

fn main() {
    let grid = ["\\    /", " \\  / ", "  \\/  "];
    println!("{}", regions(&grid));
}
