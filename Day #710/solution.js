// Day 710: Find start indices where s contains a concatenation of all equal-length
// words exactly once. Sliding window over wordLen offsets. Time O(n*wordLen).
function findSubstring(s, words) {
  const res = [];
  if (words.length === 0) return res;
  const wl = words[0].length, k = words.length, n = s.length;
  if (wl * k > n) return res;
  const need = new Map();
  for (const w of words) need.set(w, (need.get(w) || 0) + 1);
  for (let off = 0; off < wl; off++) {
    let left = off, count = 0;
    const window = new Map();
    for (let j = off; j + wl <= n; j += wl) {
      const w = s.substr(j, wl);
      if (need.has(w)) {
        window.set(w, (window.get(w) || 0) + 1); count++;
        while (window.get(w) > need.get(w)) {
          const lw = s.substr(left, wl);
          window.set(lw, window.get(lw) - 1); left += wl; count--;
        }
        if (count === k) {
          res.push(left);
          const lw = s.substr(left, wl);
          window.set(lw, window.get(lw) - 1); left += wl; count--;
        }
      } else {
        window.clear(); count = 0; left = j + wl;
      }
    }
  }
  return res.sort((a, b) => a - b);
}

const fmt = a => "[" + a.join(", ") + "]";
console.log(fmt(findSubstring("dogcatcatcodecatdog", ["cat", "dog"])));
console.log(fmt(findSubstring("barfoobazbitbyte", ["dog", "cat"])));
