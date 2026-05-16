// Concatenation of all equal-length words: sliding window per offset (0..L-1).
// Time O(|s| * L), Space O(words). Each word used exactly once.
function findSubstring(s, words) {
  const res = [];
  if (words.length === 0) return res;
  const L = words[0].length, k = words.length, n = s.length;
  if (L * k > n) return res;
  const need = new Map();
  for (const w of words) need.set(w, (need.get(w) || 0) + 1);
  for (let off = 0; off < L; off++) {
    let left = off, count = 0;
    const win = new Map();
    for (let j = off; j + L <= n; j += L) {
      const w = s.substr(j, L);
      if (need.has(w)) {
        win.set(w, (win.get(w) || 0) + 1);
        count++;
        while (win.get(w) > need.get(w)) {
          const lw = s.substr(left, L);
          win.set(lw, win.get(lw) - 1);
          left += L;
          count--;
        }
        if (count === k) {
          res.push(left);
          const lw = s.substr(left, L);
          win.set(lw, win.get(lw) - 1);
          left += L;
          count--;
        }
      } else {
        win.clear();
        count = 0;
        left = j + L;
      }
    }
  }
  res.sort((a, b) => a - b);
  return res;
}

console.log(JSON.stringify(findSubstring("dogcatcatcodecatdog", ["cat", "dog"])));
console.log(JSON.stringify(findSubstring("barfoobazbitbyte", ["dog", "cat"])));
