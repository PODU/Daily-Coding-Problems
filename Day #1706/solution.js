// Longest consecutive sequence: hash set, count runs from each sequence start. O(n) time, O(n) space.
function longestConsecutive(nums) {
  const s = new Set(nums);
  let best = 0;
  for (const n of s) {
    if (!s.has(n - 1)) {
      let cur = n, len = 1;
      while (s.has(cur + 1)) { cur++; len++; }
      best = Math.max(best, len);
    }
  }
  return best;
}

console.log(longestConsecutive([100, 4, 200, 1, 3, 2]));
