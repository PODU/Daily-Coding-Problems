// DP over rows, tracking the two smallest running totals so each house picks
// the best previous color != its own. Time O(N*K), Space O(1) extra.

function minCost(costs) {
  if (costs.length === 0) return 0;
  const K = costs[0].length;
  let prev = costs[0].slice();
  for (let i = 1; i < costs.length; i++) {
    let m1 = Infinity, m2 = Infinity, idx1 = -1;
    for (let k = 0; k < K; k++) {
      if (prev[k] < m1) { m2 = m1; m1 = prev[k]; idx1 = k; }
      else if (prev[k] < m2) m2 = prev[k];
    }
    const cur = new Array(K);
    for (let k = 0; k < K; k++) cur[k] = costs[i][k] + (k === idx1 ? m2 : m1);
    prev = cur;
  }
  return Math.min(...prev);
}

console.log(minCost([[1, 5, 3], [2, 9, 4]])); // 5
