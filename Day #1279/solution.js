// Day 1279: Lazy bartender = minimum set cover over customers.
// DP over customer bitmask. Time O(2^C * D), Space O(2^C). C=#customers, D=#drinks.
function minDrinks(preferences) {
  const customers = Object.keys(preferences);
  const C = customers.length;
  const drinkMask = new Map();
  customers.forEach((cust, i) => {
    for (const d of preferences[cust])
      drinkMask.set(d, (drinkMask.get(d) || 0) | (1 << i));
  });
  const full = (1 << C) - 1;
  const dp = new Array(1 << C).fill(Infinity);
  dp[0] = 0;
  const masks = [...drinkMask.values()];
  for (let mask = 0; mask <= full; mask++) {
    if (dp[mask] === Infinity) continue;
    for (const dm of masks) {
      const nm = mask | dm;
      if (dp[mask] + 1 < dp[nm]) dp[nm] = dp[mask] + 1;
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
