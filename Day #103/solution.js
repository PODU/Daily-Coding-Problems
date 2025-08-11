// Day 103: Shortest window containing all chars of a set via sliding window with
// a required-count and a "have all" tracker. O(n) time, O(set) space.
function minWindow(s, chars) {
  const need = new Set(chars);
  if (need.size === 0) return "";
  const count = new Map();
  let have = 0, left = 0, bestLen = Infinity, bestStart = 0;
  for (let right = 0; right < s.length; right++) {
    const ch = s[right];
    if (need.has(ch)) {
      count.set(ch, (count.get(ch) || 0) + 1);
      if (count.get(ch) === 1) have++;
    }
    while (have === need.size) {
      if (right - left + 1 < bestLen) { bestLen = right - left + 1; bestStart = left; }
      const lc = s[left];
      if (need.has(lc)) {
        count.set(lc, count.get(lc) - 1);
        if (count.get(lc) === 0) have--;
      }
      left++;
    }
  }
  return bestLen === Infinity ? null : s.slice(bestStart, bestStart + bestLen);
}

console.log(minWindow("figehaeci", new Set(['a', 'e', 'i']))); // aeci
