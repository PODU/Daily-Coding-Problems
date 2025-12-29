// Max profit single buy-then-sell: track min price so far and max profit in one pass. O(n) time, O(1) space.

function maxProfit(prices) {
  let minPrice = Infinity;
  let best = 0;
  for (const p of prices) {
    minPrice = Math.min(minPrice, p);
    best = Math.max(best, p - minPrice);
  }
  return best;
}

console.log(maxProfit([9, 11, 8, 5, 7, 10]));
