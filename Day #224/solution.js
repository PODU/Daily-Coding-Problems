// Day 224: Smallest positive integer not expressible as a subset sum (sorted array).
// Approach: greedy. Keep reachable range [1, ans-1]; if next a <= ans, extend by a, else ans is the gap.
// Time O(N), Space O(1).
function smallestNonSubsetSum(a) {
  let ans = 1; // smallest unreachable so far
  for (const x of a) {
    if (x > ans) break;
    ans += x;
  }
  return ans;
}

console.log(smallestNonSubsetSum([1, 2, 3, 10])); // 7
