// Bijective char mapping check: track s1->s2 map and set of used s2 chars; reject conflicts.
// Time O(n), Space O(unique chars).
'use strict';

function isBijective(s1, s2) {
  if (s1.length !== s2.length) return false;
  const mapping = new Map();
  const used = new Set();
  for (let i = 0; i < s1.length; i++) {
    const a = s1[i], b = s2[i];
    if (mapping.has(a)) {
      if (mapping.get(a) !== b) return false;
    } else {
      if (used.has(b)) return false;
      mapping.set(a, b);
      used.add(b);
    }
  }
  return true;
}

console.log(isBijective("abc", "bcd"));
console.log(isBijective("foo", "bar"));
