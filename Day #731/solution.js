// Day 731: Max profit from a single buy-then-sell.
// Approach: Track running minimum price and best profit in one pass.
// Time: O(n), Space: O(1).

function maxProfit(prices) {
  let minPrice = Infinity, best = 0;
  for (const p of prices) {
    minPrice = Math.min(minPrice, p);
    best = Math.max(best, p - minPrice);
  }
  return best;
}

console.log(maxProfit([9, 11, 8, 5, 7, 10])); // 5
