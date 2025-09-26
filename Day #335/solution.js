// PageRank via iterative power method, d=0.85. Iterate until convergence.
// Time: O(iters * (N + E)). Space: O(N + E).

function main() {
  const names = ["A", "B", "C", "D"];
  const N = names.length;
  const idx = {};
  names.forEach((n, i) => (idx[n] = i));
  const edges = [["A", "B"], ["A", "C"], ["B", "C"], ["C", "A"], ["D", "C"]];
  const incoming = Array.from({ length: N }, () => []);
  const out = new Array(N).fill(0);
  for (const [u, v] of edges) {
    incoming[idx[v]].push(idx[u]);
    out[idx[u]]++;
  }

  const d = 0.85;
  let score = new Array(N).fill(1.0 / N);
  for (let it = 0; it < 1000; it++) {
    const nxt = new Array(N).fill((1.0 - d) / N);
    for (let j = 0; j < N; j++)
      for (const i of incoming[j]) nxt[j] += (d * score[i]) / out[i];
    let diff = 0;
    for (let k = 0; k < N; k++) diff += Math.abs(nxt[k] - score[k]);
    score = nxt;
    if (diff < 1e-9) break;
  }
  for (let i = 0; i < N; i++) console.log(`${names[i]}: ${score[i].toFixed(4)}`);
}

main();
