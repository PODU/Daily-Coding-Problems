// Day 1027: Longest consecutive elements sequence.
// Hash set; only count runs starting at numbers with no predecessor. Time O(n), Space O(n).
function longestConsecutive(nums) {
  const s = new Set(nums);
  let best = 0;
  for (const x of s) {
    if (!s.has(x - 1)) {
      let len = 1;
      let cur = x;
      while (s.has(cur + 1)) {
        cur++;
        len++;
      }
      best = Math.max(best, len);
    }
  }
  return best;
}

console.log(longestConsecutive([100, 4, 200, 1, 3, 2]));
