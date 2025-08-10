// Day 99: Longest consecutive sequence. Hash all values; begin counting only at
// sequence starts (n-1 absent) and walk up. O(n) time, O(n) space.
function longestConsecutive(nums) {
  const s = new Set(nums);
  let best = 0;
  for (const n of s) {
    if (!s.has(n - 1)) {
      let length = 1;
      while (s.has(n + length)) length++;
      best = Math.max(best, length);
    }
  }
  return best;
}

console.log(longestConsecutive([100, 4, 200, 1, 3, 2])); // 4
