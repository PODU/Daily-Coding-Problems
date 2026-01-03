// Day 844: 2-SAT via implication graph + Kosaraju SCC.
// Clause (x OR y) adds !x->y and !y->x. SAT iff no var shares an SCC with its negation. O(V+E).
class TwoSat {
  constructor(n) {
    this.n = n;
    this.adj = Array.from({ length: 2 * n }, () => []);
  }
  static node(v, neg) { return 2 * v + (neg ? 1 : 0); }
  addClause(va, na, vb, nb) {
    const a = TwoSat.node(va, na), b = TwoSat.node(vb, nb);
    this.adj[a ^ 1].push(b);
    this.adj[b ^ 1].push(a);
  }
  solve() {
    const n2 = 2 * this.n, vis = new Array(n2).fill(false), order = [];
    const dfs1 = (s) => {
      const st = [[s, 0]];
      while (st.length) {
        const top = st[st.length - 1];
        const [u, i] = top;
        if (i === 0) vis[u] = true;
        if (i < this.adj[u].length) {
          top[1]++;
          const nx = this.adj[u][i];
          if (!vis[nx]) st.push([nx, 0]);
        } else { order.push(u); st.pop(); }
      }
    };
    for (let i = 0; i < n2; i++) if (!vis[i]) dfs1(i);
    const radj = Array.from({ length: n2 }, () => []);
    for (let u = 0; u < n2; u++) for (const v of this.adj[u]) radj[v].push(u);
    const comp = new Array(n2).fill(-1);
    let c = 0;
    for (let k = n2 - 1; k >= 0; k--) {
      const u = order[k];
      if (comp[u] === -1) {
        const st = [u]; comp[u] = c;
        while (st.length) {
          const x = st.pop();
          for (const v of radj[x]) if (comp[v] === -1) { comp[v] = c; st.push(v); }
        }
        c++;
      }
    }
    const assign = new Array(this.n);
    for (let v = 0; v < this.n; v++) {
      if (comp[2 * v] === comp[2 * v + 1]) return null;
      assign[v] = comp[2 * v] > comp[2 * v + 1];
    }
    return assign;
  }
}

const ts = new TwoSat(3); // a=0,b=1,c=2
ts.addClause(2, true, 1, false); // ¬c OR b
ts.addClause(1, false, 2, false); // b OR c
ts.addClause(1, true, 2, false); // ¬b OR c
ts.addClause(2, true, 0, true);  // ¬c OR ¬a
const r = ts.solve();
if (r === null) console.log("False");
else console.log(`a = ${r[0] ? "True" : "False"}, b = ${r[1] ? "True" : "False"}, c = ${r[2] ? "True" : "False"}`);
