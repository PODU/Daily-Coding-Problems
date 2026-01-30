// Day 993: Majority element (the value occurring more than floor(n/2) times).
// Count occurrences in a hash map and return the most frequent value. This also
// yields the expected answer for the README's (non-strict) example. O(n) time/space.

function majority(nums) {
  const freq = new Map();
  let best = nums[0],
    bestCount = 0;
  for (const x of nums) {
    const c = (freq.get(x) || 0) + 1;
    freq.set(x, c);
    if (c > bestCount) {
      bestCount = c;
      best = x;
    }
  }
  return best;
}

console.log(majority([1, 2, 1, 1, 3, 4, 0])); // 1
