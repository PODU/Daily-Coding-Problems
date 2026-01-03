// Day 844: 2-SAT via implication graph + Kosaraju SCC.
// Clause (x OR y) adds !x->y and !y->x. SAT iff no var shares an SCC with its negation. O(V+E).
struct TwoSat {
    n: usize,
    adj: Vec<Vec<usize>>,
}

impl TwoSat {
    fn new(n: usize) -> Self {
        TwoSat { n, adj: vec![Vec::new(); 2 * n] }
    }
    fn node(v: usize, neg: bool) -> usize {
        2 * v + if neg { 1 } else { 0 }
    }
    fn add_clause(&mut self, va: usize, na: bool, vb: usize, nb: bool) {
        let a = Self::node(va, na);
        let b = Self::node(vb, nb);
        self.adj[a ^ 1].push(b);
        self.adj[b ^ 1].push(a);
    }
    fn solve(&self) -> Option<Vec<bool>> {
        let n2 = 2 * self.n;
        let mut vis = vec![false; n2];
        let mut order = Vec::new();
        // iterative dfs1
        for s in 0..n2 {
            if vis[s] {
                continue;
            }
            let mut stack = vec![(s, 0usize)];
            vis[s] = true;
            while let Some(&(u, i)) = stack.last() {
                if i < self.adj[u].len() {
                    stack.last_mut().unwrap().1 += 1;
                    let nx = self.adj[u][i];
                    if !vis[nx] {
                        vis[nx] = true;
                        stack.push((nx, 0));
                    }
                } else {
                    order.push(u);
                    stack.pop();
                }
            }
        }
        let mut radj = vec![Vec::new(); n2];
        for u in 0..n2 {
            for &v in &self.adj[u] {
                radj[v].push(u);
            }
        }
        let mut comp = vec![usize::MAX; n2];
        let mut c = 0usize;
        for k in (0..n2).rev() {
            let u = order[k];
            if comp[u] == usize::MAX {
                let mut st = vec![u];
                comp[u] = c;
                while let Some(x) = st.pop() {
                    for &v in &radj[x] {
                        if comp[v] == usize::MAX {
                            comp[v] = c;
                            st.push(v);
                        }
                    }
                }
                c += 1;
            }
        }
        let mut assign = vec![false; self.n];
        for v in 0..self.n {
            if comp[2 * v] == comp[2 * v + 1] {
                return None;
            }
            assign[v] = comp[2 * v] > comp[2 * v + 1];
        }
        Some(assign)
    }
}

fn tf(b: bool) -> &'static str {
    if b { "True" } else { "False" }
}

fn main() {
    let mut ts = TwoSat::new(3); // a=0,b=1,c=2
    ts.add_clause(2, true, 1, false); // ¬c OR b
    ts.add_clause(1, false, 2, false); // b OR c
    ts.add_clause(1, true, 2, false); // ¬b OR c
    ts.add_clause(2, true, 0, true); // ¬c OR ¬a
    match ts.solve() {
        None => println!("False"),
        Some(a) => println!("a = {}, b = {}, c = {}", tf(a[0]), tf(a[1]), tf(a[2])),
    }
}
