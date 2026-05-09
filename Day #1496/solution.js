// Day 1496: Min cost to paint N houses with K colors, no two adjacent same color.
// Approach: DP tracking previous row's min & 2nd-min (+min index). Time O(N*K), Space O(1).
function minCost(costs) {
  if (costs.length === 0) return 0;
  let prevMin1 = 0, prevMin2 = 0, prevIdx = -1;
  for (const row of costs) {
    let curMin1 = Infinity, curMin2 = Infinity, curIdx = -1;
    for (let k = 0; k < row.length; k++) {
      const add = k === prevIdx ? prevMin2 : prevMin1;
      const c = row[k] + add;
      if (c < curMin1) { curMin2 = curMin1; curMin1 = c; curIdx = k; }
      else if (c < curMin2) { curMin2 = c; }
    }
    prevMin1 = curMin1; prevMin2 = curMin2; prevIdx = curIdx;
  }
  return prevMin1;
}

const costs = [[1, 5, 3], [2, 9, 4]];
console.log(minCost(costs)); // expected 5
