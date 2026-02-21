// Day 1111 - Lazy bartender (minimum set cover)
// Approach: exact set cover via DP over bitmask of covered customers.
// Time: O(D * 2^C), Space: O(2^C).

function minDrinks(preferences) {
  const customers = Object.keys(preferences).map(Number).sort((a, b) => a - b);
  const n = customers.length;
  const cidx = new Map();
  customers.forEach((c, i) => cidx.set(c, i));

  const drinkMask = new Map();
  for (const c of customers) {
    for (const d of preferences[c]) {
      drinkMask.set(d, (drinkMask.get(d) || 0) | (1 << cidx.get(c)));
    }
  }

  const full = (1 << n) - 1;
  const INF = Infinity;
  const dp = new Array(1 << n).fill(INF);
  dp[0] = 0;
  for (let s = 0; s < (1 << n); s++) {
    if (dp[s] === INF) continue;
    for (const m of drinkMask.values()) {
      const ns = s | m;
      if (dp[ns] > dp[s] + 1) dp[ns] = dp[s] + 1;
    }
  }
  return dp[full];
}

const preferences = {
  0: [0, 1, 3, 6],
  1: [1, 4, 7],
  2: [2, 4, 7, 5],
  3: [3, 2, 5],
  4: [5, 8],
};
console.log(minDrinks(preferences)); // 2
