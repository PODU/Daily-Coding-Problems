// Day 831: All start indices of substrings that are a concatenation of every word once.
// Sliding window per offset 0..L-1 with hashmap word counts. O(n * L) ~ O(n) total.
"use strict";

function findSubstring(s, words) {
  const res = [];
  if (!words.length || !s.length) return res;
  const L = words[0].length, m = words.length, n = s.length;
  if (L * m > n) return res;
  const need = new Map();
  for (const w of words) need.set(w, (need.get(w) || 0) + 1);

  for (let offset = 0; offset < L; offset++) {
    let left = offset, count = 0;
    const have = new Map();
    for (let right = offset; right + L <= n; right += L) {
      const w = s.substr(right, L);
      if (need.has(w)) {
        have.set(w, (have.get(w) || 0) + 1);
        count++;
        while (have.get(w) > need.get(w)) {
          const lw = s.substr(left, L);
          have.set(lw, have.get(lw) - 1);
          left += L;
          count--;
        }
        if (count === m) {
          res.push(left);
          const lw = s.substr(left, L);
          have.set(lw, have.get(lw) - 1);
          left += L;
          count--;
        }
      } else {
        have.clear();
        count = 0;
        left = right + L;
      }
    }
  }
  res.sort((a, b) => a - b);
  return res;
}

function fmt(v) {
  return "[" + v.join(", ") + "]";
}

function main() {
  console.log(fmt(findSubstring("dogcatcatcodecatdog", ["cat", "dog"]))); // [0, 13]
  console.log(fmt(findSubstring("barfoobazbitbyte", ["dog", "cat"])));    // []
}

main();
