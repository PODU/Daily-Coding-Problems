// Day 485: Longest substring with at most k distinct characters.
// Sliding window + count map; expand right, shrink left when distinct > k. Time O(n), Space O(k).
function longestKDistinct(s, k) {
  if (k <= 0) return 0;
  const count = new Map();
  let left = 0;
  let best = 0;
  for (let right = 0; right < s.length; right++) {
    const c = s[right];
    count.set(c, (count.get(c) || 0) + 1);
    while (count.size > k) {
      const lc = s[left];
      const nc = count.get(lc) - 1;
      if (nc === 0) count.delete(lc);
      else count.set(lc, nc);
      left++;
    }
    best = Math.max(best, right - left + 1);
  }
  return best;
}

console.log(longestKDistinct("abcba", 2)); // 3
