// Day 413: Ordered ways to climb a staircase with allowed step sizes X.
// DP: ways[n] = sum over x in X of ways[n-x]. Time O(N*|X|), Space O(N).
function countWays(n, steps) {
  const ways = new Array(n + 1).fill(0);
  ways[0] = 1;
  for (let i = 1; i <= n; i++)
    for (const x of steps)
      if (x <= i) ways[i] += ways[i - x];
  return ways[n];
}

console.log(countWays(4, [1, 2]));      // 5
console.log(countWays(10, [1, 3, 5]));  // generalized X={1,3,5}
