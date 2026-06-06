// Day 1622: Longest substring with at most k distinct characters.
// Sliding window with a count map. Time O(n), Space O(k).
function longestKDistinct(s, k) {
  if (k <= 0) return "";
  const cnt = new Map();
  let left = 0, bestL = 0, bestLen = 0;
  for (let right = 0; right < s.length; right++) {
    const c = s[right];
    cnt.set(c, (cnt.get(c) || 0) + 1);
    while (cnt.size > k) {
      const lc = s[left];
      cnt.set(lc, cnt.get(lc) - 1);
      if (cnt.get(lc) === 0) cnt.delete(lc);
      left++;
    }
    if (right - left + 1 > bestLen) { bestLen = right - left + 1; bestL = left; }
  }
  return s.slice(bestL, bestL + bestLen);
}

console.log('"' + longestKDistinct("abcba", 2) + '"');
