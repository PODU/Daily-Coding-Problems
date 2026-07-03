// Day 1757: Count ordered ways to climb N stairs (steps 1 or 2 -> Fibonacci).
// Generalized to a step set X via DP: ways[i] = sum of ways[i-x]. O(N*|X|) time, O(N) space.
'use strict';

function climbWays(n, steps) {
  const ways = new Array(n + 1).fill(0);
  ways[0] = 1;
  for (let i = 1; i <= n; i++)
    for (const x of steps)
      if (i - x >= 0) ways[i] += ways[i - x];
  return ways[n];
}

function main() {
  const N = 4;
  console.log(climbWays(N, [1, 2])); // 5
  console.log("Generalized X={1,3,5}, N=4:", climbWays(N, [1, 3, 5]));
}

main();
