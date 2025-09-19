// Exact set cover via BFS/DP over bitmask of customers; each drink = bitmask of customers accepting it.
// Time: O(2^m * drinks), Space: O(2^m)  (m = number of customers, small).
function minDrinks(preferences) {
  const customers = Object.keys(preferences);
  const idx = {};
  customers.forEach((c, i) => (idx[c] = i));
  const m = customers.length;
  const full = (1 << m) - 1;

  const drinkMask = {};
  for (const c of customers) {
    for (const d of preferences[c]) {
      drinkMask[d] = (drinkMask[d] || 0) | (1 << idx[c]);
    }
  }

  const dp = new Map([[0, 0]]);
  const q = [0];
  while (q.length) {
    const mask = q.shift();
    if (mask === full) continue;
    for (const dm of Object.values(drinkMask)) {
      const nm = mask | dm;
      if (!dp.has(nm) || dp.get(nm) > dp.get(mask) + 1) {
        dp.set(nm, dp.get(mask) + 1);
        q.push(nm);
      }
    }
  }
  return dp.get(full);
}

const preferences = { 0: [0, 1, 3, 6], 1: [1, 4, 7], 2: [2, 4, 7, 5], 3: [3, 2, 5], 4: [5, 8] };
console.log(minDrinks(preferences));
