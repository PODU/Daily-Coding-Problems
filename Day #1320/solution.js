// Longest substring with at most k distinct characters via a sliding window
// with a char-count map. Time O(n), Space O(k).
'use strict';

function longestKDistinct(s, k) {
  const cnt = new Map();
  let left = 0, bestStart = 0, bestLen = 0;
  for (let right = 0; right < s.length; right++) {
    cnt.set(s[right], (cnt.get(s[right]) || 0) + 1);
    while (cnt.size > k) {
      const l = s[left];
      cnt.set(l, cnt.get(l) - 1);
      if (cnt.get(l) === 0) cnt.delete(l);
      left++;
    }
    if (right - left + 1 > bestLen) { bestLen = right - left + 1; bestStart = left; }
  }
  return s.slice(bestStart, bestStart + bestLen);
}

const sub = longestKDistinct("abcba", 2);
console.log(`The longest substring with k distinct characters is "${sub}".`);
