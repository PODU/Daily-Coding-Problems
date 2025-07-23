// Paint House: DP tracking min cost per color using min1/min2 trick.
// Time O(N*K), Space O(1) extra.
function minCost(cost) {
  if (cost.length === 0) return 0;
  const K = cost[0].length;
  let prev = cost[0].slice();
  for (let i = 1; i < cost.length; i++) {
    let min1 = -1, min2 = -1;
    for (let k = 0; k < K; k++) {
      if (min1 === -1 || prev[k] < prev[min1]) { min2 = min1; min1 = k; }
      else if (min2 === -1 || prev[k] < prev[min2]) { min2 = k; }
    }
    const cur = new Array(K);
    for (let k = 0; k < K; k++) {
      const best = k === min1 ? prev[min2] : prev[min1];
      cur[k] = cost[i][k] + best;
    }
    prev = cur;
  }
  return Math.min(...prev);
}

const cost = [[1, 5, 3], [2, 9, 4]];
console.log("Minimum cost = " + minCost(cost));
