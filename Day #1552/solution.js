// Max profit single buy-then-sell: track running min price and best (price - min). Time O(n), Space O(1).

function maxProfit(prices) {
  if (prices.length === 0) return 0;
  let minPrice = prices[0];
  let best = 0;
  for (const p of prices) {
    minPrice = Math.min(minPrice, p);
    best = Math.max(best, p - minPrice);
  }
  return best;
}

function main() {
  const prices = [9, 11, 8, 5, 7, 10];
  console.log(maxProfit(prices));
}

main();
