// Day 1804: Find a duplicate in array of n+1 elements from {1..n} using a Set.
// O(n) time, O(n) space.
function findDuplicate(nums) {
  const seen = new Set();
  for (const x of nums) {
    if (seen.has(x)) return x;
    seen.add(x);
  }
  return -1; // no duplicate (won't happen per problem constraints)
}

function main() {
  const nums = [1, 3, 4, 2, 2];
  console.log(findDuplicate(nums)); // expected 2
}

main();
