// Day 717: Min cost to paint N houses, K colors, no two adjacent same color.
// DP over rows tracking best & second-best previous costs. Time O(N*K), space O(1).
function minCost(costs) {
  if (costs.length === 0) return 0;
  const K = costs[0].length;
  let prev = costs[0].slice();
  for (let i = 1; i < costs.length; i++) {
    let m1 = Infinity, m2 = Infinity, idx = -1;
    for (let k = 0; k < K; k++) {
      if (prev[k] < m1) { m2 = m1; m1 = prev[k]; idx = k; }
      else if (prev[k] < m2) m2 = prev[k];
    }
    const cur = new Array(K);
    for (let k = 0; k < K; k++) cur[k] = costs[i][k] + (k === idx ? m2 : m1);
    prev = cur;
  }
  return Math.min(...prev);
}

console.log(minCost([[1, 5, 3], [2, 9, 4]]));
