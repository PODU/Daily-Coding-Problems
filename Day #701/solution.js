// Day 701: Minimum drinks to satisfy every customer (minimum set cover).
// Approach: each drink -> bitmask of customers it satisfies; DP over customer
// masks for fewest drinks covering all. Time O(2^C * D), Space O(2^C).
function minDrinks(prefs) {
  const custs = Object.keys(prefs);
  const idx = new Map(custs.map((c, i) => [c, i]));
  const C = custs.length;
  const drinkMask = new Map();
  for (const cust of custs)
    for (const d of prefs[cust])
      drinkMask.set(d, (drinkMask.get(d) || 0) | (1 << idx.get(cust)));
  const full = (1 << C) - 1;
  const dp = new Array(full + 1).fill(Infinity);
  dp[0] = 0;
  for (let mask = 0; mask <= full; mask++) {
    if (dp[mask] === Infinity) continue;
    for (const dm of drinkMask.values()) {
      const nm = mask | dm;
      if (dp[mask] + 1 < dp[nm]) dp[nm] = dp[mask] + 1;
    }
  }
  return dp[full];
}

const prefs = {
  0: [0, 1, 3, 6],
  1: [1, 4, 7],
  2: [2, 4, 7, 5],
  3: [3, 2, 5],
  4: [5, 8],
};
console.log(minDrinks(prefs)); // 2
