// Day 1476: Max profit from a single buy-then-sell.
// Track the minimum price so far and the best profit in one pass.
// Time O(N), Space O(1).

function maxProfit(prices) {
  let minPrice = Infinity, best = 0;
  for (const p of prices) {
    if (p < minPrice) minPrice = p;
    else if (p - minPrice > best) best = p - minPrice;
  }
  return best;
}

console.log(maxProfit([9, 11, 8, 5, 7, 10])); // 5
