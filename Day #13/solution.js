// Longest substring with at most k distinct chars: sliding window + count map.
// Time: O(n), Space: O(k).
function longestKDistinct(s, k) {
  if (k === 0) return 0;
  const cnt = new Map();
  let left = 0, best = 0;
  for (let right = 0; right < s.length; right++) {
    const c = s[right];
    cnt.set(c, (cnt.get(c) || 0) + 1);
    while (cnt.size > k) {
      const lc = s[left];
      cnt.set(lc, cnt.get(lc) - 1);
      if (cnt.get(lc) === 0) cnt.delete(lc);
      left++;
    }
    best = Math.max(best, right - left + 1);
  }
  return best;
}

console.log(longestKDistinct("abcba", 2)); // 3
