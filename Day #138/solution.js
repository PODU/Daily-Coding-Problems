// Greedy on canonical US denominations {25,10,5,1}: take largest coin each step.
// Time O(D) where D = #denominations; Space O(1).
function minCoins(n) {
  let count = 0;
  for (const c of [25, 10, 5, 1]) {
    count += Math.floor(n / c);
    n %= c;
  }
  return count;
}

console.log(minCoins(16)); // 3
