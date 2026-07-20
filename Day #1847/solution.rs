// Day 1847: 2-SAT solver. Build implication graph, find SCCs (Kosaraju), check x and ¬x differ.
// Time O(V + C), Space O(V + C). Literal node = 2*var + (negated?1:0); neg(x)=x^1.

struct TwoSat {
    n: usize,
    adj: Vec<Vec<usize>>,
    adj_t: Vec<Vec<usize>>,
}

impl TwoSat {
    fn new(n: usize) -> Self {
        TwoSat { n, adj: vec![vec![]; 2 * n], adj_t: vec![vec![]; 2 * n] }
    }
    fn add_or(&mut self, u: usize, v: usize) {
        self.adj[u ^ 1].push(v);
        self.adj_t[v].push(u ^ 1);
        self.adj[v ^ 1].push(u);
        self.adj_t[u].push(v ^ 1);
    }
    fn dfs1(&self, v: usize, used: &mut [bool], order: &mut Vec<usize>) {
        used[v] = true;
        for &to in &self.adj[v] {
            if !used[to] {
                self.dfs1(to, used, order);
            }
        }
        order.push(v);
    }
    fn dfs2(&self, v: usize, c: usize, comp: &mut [i32]) {
        comp[v] = c as i32;
        for &to in &self.adj_t[v] {
            if comp[to] == -1 {
                self.dfs2(to, c, comp);
            }
        }
    }
    fn solve(&self) -> Option<Vec<bool>> {
        let big = 2 * self.n;
        let mut used = vec![false; big];
        let mut order = Vec::new();
        for i in 0..big {
            if !used[i] {
                self.dfs1(i, &mut used, &mut order);
            }
        }
        let mut comp = vec![-1i32; big];
        let mut c = 0;
        for &v in order.iter().rev() {
            if comp[v] == -1 {
                self.dfs2(v, c, &mut comp);
                c += 1;
            }
        }
        let mut res = vec![false; self.n];
        for i in 0..self.n {
            if comp[2 * i] == comp[2 * i + 1] {
                return None;
            }
            res[i] = comp[2 * i] > comp[2 * i + 1];
        }
        Some(res)
    }
}

fn lit(v: usize, neg: usize) -> usize {
    2 * v + neg
}

fn main() {
    let mut ts = TwoSat::new(3); // a=0, b=1, c=2
    ts.add_or(lit(2, 1), lit(1, 0)); // (¬c OR b)
    ts.add_or(lit(1, 0), lit(2, 0)); // (b OR c)
    ts.add_or(lit(1, 1), lit(2, 0)); // (¬b OR c)
    ts.add_or(lit(2, 1), lit(0, 1)); // (¬c OR ¬a)

    match ts.solve() {
        None => println!("False"),
        Some(res) => {
            let names = ['a', 'b', 'c'];
            let parts: Vec<String> = (0..3)
                .map(|i| format!("{} = {}", names[i], if res[i] { "True" } else { "False" }))
                .collect();
            println!("{}", parts.join(", "));
        }
    }
}
