// Day 1002: Smallest positive integer not expressible as a subset sum (sorted array).
// If the next element x <= res (smallest unreachable, init 1) extend to res+x,
// else res is the answer. O(N) time, O(1) space.

function smallestNonSubsetSum(nums) {
  let res = 1;
  for (const x of nums) {
    if (x > res) break;
    res += x;
  }
  return res;
}

console.log(smallestNonSubsetSum([1, 2, 3, 10])); // 7
