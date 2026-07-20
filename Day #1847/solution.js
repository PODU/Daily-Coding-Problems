// Day 1847: 2-SAT solver. Build implication graph, find SCCs (Kosaraju), check x and ¬x differ.
// Time O(V + C), Space O(V + C). Literal node = 2*var + (negated?1:0); neg(x)=x^1.

class TwoSat {
  constructor(n) {
    this.n = n;
    this.adj = Array.from({ length: 2 * n }, () => []);
    this.adjT = Array.from({ length: 2 * n }, () => []);
  }
  addOr(u, v) {
    this.adj[u ^ 1].push(v);
    this.adjT[v].push(u ^ 1);
    this.adj[v ^ 1].push(u);
    this.adjT[u].push(v ^ 1);
  }
  solve() {
    const N = 2 * this.n;
    const used = new Array(N).fill(false);
    const order = [];
    const dfs1 = (v) => {
      used[v] = true;
      for (const to of this.adj[v]) if (!used[to]) dfs1(to);
      order.push(v);
    };
    for (let i = 0; i < N; i++) if (!used[i]) dfs1(i);
    const comp = new Array(N).fill(-1);
    const dfs2 = (v, c) => {
      comp[v] = c;
      for (const to of this.adjT[v]) if (comp[to] === -1) dfs2(to, c);
    };
    let c = 0;
    for (let i = N - 1; i >= 0; i--) {
      const v = order[i];
      if (comp[v] === -1) dfs2(v, c++);
    }
    const res = new Array(this.n);
    for (let i = 0; i < this.n; i++) {
      if (comp[2 * i] === comp[2 * i + 1]) return null;
      res[i] = comp[2 * i] > comp[2 * i + 1];
    }
    return res;
  }
}

const lit = (v, neg) => 2 * v + neg;

function main() {
  const ts = new TwoSat(3); // a=0, b=1, c=2
  ts.addOr(lit(2, 1), lit(1, 0)); // (¬c OR b)
  ts.addOr(lit(1, 0), lit(2, 0)); // (b OR c)
  ts.addOr(lit(1, 1), lit(2, 0)); // (¬b OR c)
  ts.addOr(lit(2, 1), lit(0, 1)); // (¬c OR ¬a)

  const res = ts.solve();
  if (res === null) {
    console.log("False");
    return;
  }
  const names = "abc";
  console.log(res.map((b, i) => `${names[i]} = ${b ? "True" : "False"}`).join(", "));
}

main();
