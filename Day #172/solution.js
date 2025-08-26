// Substring concatenation of all words: sliding window over wordLen offsets with hash-map counts.
// Time O(n * wordLen), Space O(words * wordLen).
function findSubstring(s, words) {
  const res = [];
  if (!words.length || !s.length) return res;
  const wl = words[0].length, nw = words.length, total = wl * nw, n = s.length;
  if (total > n) return res;
  const need = new Map();
  for (const w of words) need.set(w, (need.get(w) || 0) + 1);
  for (let i = 0; i < wl; i++) {
    let left = i, count = 0;
    const window = new Map();
    for (let j = i; j + wl <= n; j += wl) {
      const w = s.substr(j, wl);
      if (need.has(w)) {
        window.set(w, (window.get(w) || 0) + 1); count++;
        while (window.get(w) > need.get(w)) {
          const lw = s.substr(left, wl);
          window.set(lw, window.get(lw) - 1); count--; left += wl;
        }
        if (count === nw) {
          res.push(left);
          const lw = s.substr(left, wl);
          window.set(lw, window.get(lw) - 1); count--; left += wl;
        }
      } else {
        window.clear(); count = 0; left = j + wl;
      }
    }
  }
  return res.sort((a, b) => a - b);
}

console.log("[" + findSubstring("dogcatcatcodecatdog", ["cat", "dog"]).join(", ") + "]");
console.log("[" + findSubstring("barfoobazbitbyte", ["dog", "cat"]).join(", ") + "]");
