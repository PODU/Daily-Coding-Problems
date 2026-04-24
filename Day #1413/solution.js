// Day 1413: shortest substring of s containing all characters in a set.
// Approach: sliding window, expand right then shrink left while valid. O(n) time, O(k) space.

function shortestSubstring(s, want) {
  const need = new Map();
  for (const c of want) need.set(c, (need.get(c) || 0) + 1);
  const required = need.size;
  const win = new Map();
  let formed = 0, bestLen = Infinity, bestL = 0, l = 0;
  for (let r = 0; r < s.length; r++) {
    const c = s[r];
    if (need.has(c)) {
      win.set(c, (win.get(c) || 0) + 1);
      if (win.get(c) === need.get(c)) formed++;
    }
    while (formed === required) {
      if (r - l + 1 < bestLen) { bestLen = r - l + 1; bestL = l; }
      const lc = s[l];
      if (need.has(lc)) {
        win.set(lc, win.get(lc) - 1);
        if (win.get(lc) < need.get(lc)) formed--;
      }
      l++;
    }
  }
  return bestLen === Infinity ? null : s.slice(bestL, bestL + bestLen);
}

console.log(shortestSubstring("figehaeci", new Set(["a", "e", "i"]))); // aeci
