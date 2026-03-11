// 2-SAT via implication graph + Tarjan SCC. Node 2*v=(v true), 2*v+1=(v false).
// Clause (x OR y): add edges ~x->y and ~y->x. UNSAT iff x and ~x share an SCC.
// Pick literal whose SCC is the sink; verify against clauses. Time O(V+C).

struct TwoSat {
    n: usize,
    adj: Vec<Vec<usize>>,
    clauses: Vec<(usize, bool, usize, bool)>,
    comp: Vec<i32>,
    low: Vec<i32>,
    num: Vec<i32>,
    on_stk: Vec<bool>,
    stk: Vec<usize>,
    idx: i32,
    scc: i32,
}

impl TwoSat {
    fn new(n: usize) -> Self {
        TwoSat {
            n,
            adj: vec![Vec::new(); 2 * n],
            clauses: Vec::new(),
            comp: vec![-1; 2 * n],
            low: vec![0; 2 * n],
            num: vec![-1; 2 * n],
            on_stk: vec![false; 2 * n],
            stk: Vec::new(),
            idx: 0,
            scc: 0,
        }
    }

    fn lit(v: usize, neg: bool) -> usize {
        2 * v + if neg { 1 } else { 0 }
    }

    fn clause(&mut self, v1: usize, n1: bool, v2: usize, n2: bool) {
        let a = Self::lit(v1, !n1);
        let b = Self::lit(v2, n2);
        self.adj[a].push(b);
        let c = Self::lit(v2, !n2);
        let d = Self::lit(v1, n1);
        self.adj[c].push(d);
        self.clauses.push((v1, n1, v2, n2));
    }

    fn tarjan(&mut self, u: usize) {
        self.low[u] = self.idx;
        self.num[u] = self.idx;
        self.idx += 1;
        self.stk.push(u);
        self.on_stk[u] = true;
        for i in 0..self.adj[u].len() {
            let w = self.adj[u][i];
            if self.num[w] == -1 {
                self.tarjan(w);
                self.low[u] = self.low[u].min(self.low[w]);
            } else if self.on_stk[w] {
                self.low[u] = self.low[u].min(self.num[w]);
            }
        }
        if self.low[u] == self.num[u] {
            loop {
                let x = self.stk.pop().unwrap();
                self.on_stk[x] = false;
                self.comp[x] = self.scc;
                if x == u {
                    break;
                }
            }
            self.scc += 1;
        }
    }

    fn satisfies(&self, val: &[bool]) -> bool {
        for &(v1, n1, v2, n2) in &self.clauses {
            let a = val[v1] != n1;
            let b = val[v2] != n2;
            if !(a || b) {
                return false;
            }
        }
        true
    }

    fn solve(&mut self) -> Option<Vec<bool>> {
        for i in 0..2 * self.n {
            if self.num[i] == -1 {
                self.tarjan(i);
            }
        }
        for v in 0..self.n {
            if self.comp[Self::lit(v, false)] == self.comp[Self::lit(v, true)] {
                return None;
            }
        }
        let mut val: Vec<bool> = (0..self.n)
            .map(|v| self.comp[Self::lit(v, false)] < self.comp[Self::lit(v, true)])
            .collect();
        if !self.satisfies(&val) {
            val = (0..self.n)
                .map(|v| self.comp[Self::lit(v, true)] < self.comp[Self::lit(v, false)])
                .collect();
        }
        Some(val)
    }
}

fn main() {
    let mut ts = TwoSat::new(3); // a=0, b=1, c=2
    // (~c OR b), (b OR c), (~b OR c), (~c OR ~a)
    ts.clause(2, true, 1, false);
    ts.clause(1, false, 2, false);
    ts.clause(1, true, 2, false);
    ts.clause(2, true, 0, true);

    match ts.solve() {
        None => println!("UNSATISFIABLE"),
        Some(val) => {
            let s = |b: bool| if b { "True" } else { "False" };
            println!("a = {}, b = {}, c = {}", s(val[0]), s(val[1]), s(val[2]));
        }
    }
}
