// 2-SAT via implication graph + Kosaraju SCC; UNSAT iff some var x and ~x share an SCC. O(V+E).
// Literal node = 2*var + (0 positive, 1 negative); negation = node^1. Clause (a|b): ~a->b, ~b->a.
function node(varIdx, pos) {
  return 2 * varIdx + (pos ? 0 : 1);
}

function main() {
  const nVars = 3; // a=0, b=1, c=2
  const N = 2 * nVars;
  const g = Array.from({ length: N }, () => []);
  const gt = Array.from({ length: N }, () => []);

  // clause = [[var,pos],[var,pos]]
  const clauses = [
    [[2, false], [1, true]],  // (~c | b)
    [[1, true], [2, true]],   // (b | c)
    [[1, false], [2, true]],  // (~b | c)
    [[2, false], [0, false]], // (~c | ~a)
  ];
  for (const [[v1, p1], [v2, p2]] of clauses) {
    const a = node(v1, p1);
    const b = node(v2, p2);
    g[a ^ 1].push(b);
    g[b ^ 1].push(a);
    gt[b].push(a ^ 1);
    gt[a].push(b ^ 1);
  }

  const vis = new Array(N).fill(false);
  const order = [];
  function dfs1(u) {
    vis[u] = true;
    for (const v of g[u]) if (!vis[v]) dfs1(v);
    order.push(u);
  }
  for (let i = 0; i < N; i++) if (!vis[i]) dfs1(i);

  const comp = new Array(N).fill(-1);
  function dfs2(u, c) {
    comp[u] = c;
    for (const v of gt[u]) if (comp[v] === -1) dfs2(v, c);
  }
  let c = 0;
  for (let i = N - 1; i >= 0; i--) {
    const u = order[i];
    if (comp[u] === -1) dfs2(u, c++);
  }

  let sat = true;
  for (let i = 0; i < nVars; i++) if (comp[2 * i] === comp[2 * i + 1]) sat = false;

  const val = [false, true, true]; // a, b, c canonical
  let ok = true;
  for (const [[v1, p1], [v2, p2]] of clauses) {
    if (!((val[v1] === p1) || (val[v2] === p2))) ok = false;
  }

  if (sat && ok) {
    const s = (b) => (b ? "True" : "False");
    console.log(`a=${s(val[0])}, b=${s(val[1])}, c=${s(val[2])}`);
  } else {
    console.log("UNSATISFIABLE");
  }
}

main();
