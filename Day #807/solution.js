// Day 807: Longest substring with at most k distinct characters.
// Sliding window + char count map; shrink left when distinct > k. Time O(N), Space O(k).

function longestKDistinct(s, k) {
  if (k === 0) return 0;
  const cnt = new Map();
  let left = 0, best = 0;
  for (let right = 0; right < s.length; right++) {
    const c = s[right];
    cnt.set(c, (cnt.get(c) || 0) + 1);
    while (cnt.size > k) {
      const l = s[left];
      cnt.set(l, cnt.get(l) - 1);
      if (cnt.get(l) === 0) cnt.delete(l);
      left++;
    }
    best = Math.max(best, right - left + 1);
  }
  return best;
}

console.log(longestKDistinct("abcba", 2)); // 3
