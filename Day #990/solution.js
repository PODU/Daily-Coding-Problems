// Day 990: Count ordered ways to climb N steps using step sizes from set X.
// Bottom-up DP: ways[i] = sum over x in X of ways[i-x]. O(N*|X|) time, O(N) space.

function climbWays(n, X) {
  const ways = new Array(n + 1).fill(0);
  ways[0] = 1;
  for (let i = 1; i <= n; i++)
    for (const x of X)
      if (i - x >= 0) ways[i] += ways[i - x];
  return ways[n];
}

console.log("N=4, X={1,2}:", climbWays(4, [1, 2]));       // expected 5
console.log("N=4, X={1,3,5}:", climbWays(4, [1, 3, 5]));  // generalized
