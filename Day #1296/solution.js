// Day 1296: Smallest positive integer not expressible as a subset sum of a sorted array.
// Greedy: track reachable range [1..r]; if next a[i] <= r+1 extend, else answer r+1. O(N) time.
function smallestNonSubsetSum(a) {
  let r = 0; // can form every value in [1..r]
  for (const x of a) {
    if (x > r + 1) break;
    r += x;
  }
  return r + 1;
}

console.log(smallestNonSubsetSum([1, 2, 3, 10])); // 7
