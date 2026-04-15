// Max profit, unlimited transactions with a fixed fee per completed sale.
// DP states cash/hold tracked greedily. Time O(n), Space O(1).
'use strict';

function maxProfit(prices, fee) {
  let cash = 0;
  let hold = -Infinity;
  for (const p of prices) {
    hold = Math.max(hold, cash - p);
    cash = Math.max(cash, hold + p - fee);
  }
  return cash;
}

const prices = [1, 3, 2, 8, 4, 10];
const fee = 2;
console.log(maxProfit(prices, fee));
