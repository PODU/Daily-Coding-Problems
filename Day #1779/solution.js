// Minimum coins for {1,5,10,25} via greedy (optimal for this canonical system).
// Time: O(1), Space: O(1).
function minCoins(n) {
  return Math.floor(n / 25) + Math.floor((n % 25) / 10)
    + Math.floor((n % 25 % 10) / 5) + (n % 25 % 10 % 5);
}

console.log(minCoins(16));
