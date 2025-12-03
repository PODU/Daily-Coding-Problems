// 2-SAT: implication graph + iterative Tarjan SCC. Sat iff no var shares SCC with its negation.
// Time O(n + m), Space O(n + m).

struct TwoSat {
    n: usize,
    g: Vec<Vec<usize>>,
    idx: Vec<i32>,
    low: Vec<i32>,
    comp: Vec<i32>,
    onstk: Vec<bool>,
    stk: Vec<usize>,
    cnt: i32,
    cc: i32,
}

impl TwoSat {
    fn new(vars: usize) -> Self {
        TwoSat {
            n: vars,
            g: vec![Vec::new(); 2 * vars],
            idx: vec![-1; 2 * vars],
            low: vec![0; 2 * vars],
            comp: vec![-1; 2 * vars],
            onstk: vec![false; 2 * vars],
            stk: Vec::new(),
            cnt: 0,
            cc: 0,
        }
    }

    fn node(v: usize, sign: bool) -> usize {
        2 * v + if sign { 0 } else { 1 }
    }
    fn neg(x: usize) -> usize {
        x ^ 1
    }

    fn add_clause(&mut self, v1: usize, s1: bool, v2: usize, s2: bool) {
        let x = Self::node(v1, s1);
        let y = Self::node(v2, s2);
        self.g[Self::neg(x)].push(y);
        self.g[Self::neg(y)].push(x);
    }

    fn tarjan(&mut self, start: usize) {
        let mut work: Vec<(usize, usize)> = vec![(start, 0)];
        while let Some(&(v, pi)) = work.last() {
            if pi == 0 {
                self.idx[v] = self.cnt;
                self.low[v] = self.cnt;
                self.cnt += 1;
                self.stk.push(v);
                self.onstk[v] = true;
            }
            let mut recurse = false;
            let mut i = pi;
            while i < self.g[v].len() {
                let w = self.g[v][i];
                if self.idx[w] == -1 {
                    let last = work.len() - 1;
                    work[last].1 = i + 1;
                    work.push((w, 0));
                    recurse = true;
                    break;
                } else if self.onstk[w] {
                    if self.idx[w] < self.low[v] {
                        self.low[v] = self.idx[w];
                    }
                }
                i += 1;
            }
            if recurse {
                continue;
            }
            if self.low[v] == self.idx[v] {
                loop {
                    let w = self.stk.pop().unwrap();
                    self.onstk[w] = false;
                    self.comp[w] = self.cc;
                    if w == v {
                        break;
                    }
                }
                self.cc += 1;
            }
            work.pop();
            if let Some(&(pv, _)) = work.last() {
                if self.low[v] < self.low[pv] {
                    self.low[pv] = self.low[v];
                }
            }
        }
    }

    fn solve(&mut self) -> Option<Vec<bool>> {
        for i in 0..2 * self.n {
            if self.idx[i] == -1 {
                self.tarjan(i);
            }
        }
        let mut assign = vec![false; self.n];
        for v in 0..self.n {
            if self.comp[2 * v] == self.comp[2 * v + 1] {
                return None;
            }
            assign[v] = self.comp[2 * v] < self.comp[2 * v + 1];
        }
        Some(assign)
    }
}

fn main() {
    // Variables a=0, b=1, c=2. sign true=positive, false=negated.
    // (!c OR b) AND (b OR c) AND (!b OR c) AND (!c OR !a)
    let mut ts = TwoSat::new(3);
    ts.add_clause(2, false, 1, true);
    ts.add_clause(1, true, 2, true);
    ts.add_clause(1, false, 2, true);
    ts.add_clause(2, false, 0, false);

    match ts.solve() {
        None => println!("UNSATISFIABLE"),
        Some(assign) => {
            let names = ["a", "b", "c"];
            let parts: Vec<String> = (0..3)
                .map(|i| format!("{} = {}", names[i], if assign[i] { "True" } else { "False" }))
                .collect();
            println!("{}", parts.join(", "));
        }
    }
}
