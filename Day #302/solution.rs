// Day 302: Count regions divided by slashes. Split each cell into 4 triangles,
// union-find adjacent triangles. Time O(N*M*alpha), space O(N*M).
struct DSU { p: Vec<usize> }
impl DSU {
    fn new(n: usize) -> Self { DSU { p: (0..n).collect() } }
    fn find(&mut self, x: usize) -> usize {
        let mut x = x;
        while self.p[x] != x { self.p[x] = self.p[self.p[x]]; x = self.p[x]; }
        x
    }
    fn union(&mut self, a: usize, b: usize) { let ra = self.find(a); let rb = self.find(b); self.p[ra] = rb; }
}

fn count_regions(grid: &[&str]) -> usize {
    let n = grid.len();
    let m = grid.iter().map(|r| r.len()).max().unwrap();
    let g: Vec<Vec<char>> = grid.iter().map(|r| {
        let mut v: Vec<char> = r.chars().collect();
        while v.len() < m { v.push(' '); }
        v
    }).collect();
    let mut d = DSU::new(n * m * 4);
    let idx = |i: usize, j: usize, t: usize| (i * m + j) * 4 + t; // 0=top,1=right,2=bottom,3=left
    for i in 0..n {
        for j in 0..m {
            match g[i][j] {
                '/' => { d.union(idx(i,j,0), idx(i,j,3)); d.union(idx(i,j,1), idx(i,j,2)); }
                '\\' => { d.union(idx(i,j,0), idx(i,j,1)); d.union(idx(i,j,2), idx(i,j,3)); }
                _ => { d.union(idx(i,j,0), idx(i,j,1)); d.union(idx(i,j,1), idx(i,j,2)); d.union(idx(i,j,2), idx(i,j,3)); }
            }
            if j + 1 < m { d.union(idx(i,j,1), idx(i,j+1,3)); }
            if i + 1 < n { d.union(idx(i,j,2), idx(i+1,j,0)); }
        }
    }
    (0..n * m * 4).filter(|&x| d.find(x) == x).count()
}

fn main() {
    let grid = ["\\    /", " \\  /", "  \\/"];
    println!("{}", count_regions(&grid)); // 3
}
