// Day 1029: Minimum coins for n cents with {1,5,10,25}. Greedy is optimal for
// this canonical US denomination set. Time O(#denominations), Space O(1).
function minCoins(n) {
  let count = 0;
  for (const c of [25, 10, 5, 1]) {
    count += Math.floor(n / c);
    n %= c;
  }
  return count;
}

console.log(minCoins(16));
