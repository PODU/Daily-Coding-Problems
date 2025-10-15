// Day 437: Shortest substring containing all chars of a set via sliding window.
// O(N) time, O(set) space.

function shortestSubstring(s, chars) {
  if (chars.size === 0) return "";
  const need = new Map();
  for (const c of chars) need.set(c, 0);
  const required = chars.size;
  let formed = 0, bestLen = Infinity, bestL = 0, l = 0;
  for (let r = 0; r < s.length; r++) {
    const c = s[r];
    if (need.has(c)) {
      if (need.get(c) === 0) formed++;
      need.set(c, need.get(c) + 1);
    }
    while (formed === required) {
      if (r - l + 1 < bestLen) { bestLen = r - l + 1; bestL = l; }
      const lc = s[l];
      if (need.has(lc)) {
        need.set(lc, need.get(lc) - 1);
        if (need.get(lc) === 0) formed--;
      }
      l++;
    }
  }
  return bestLen === Infinity ? null : s.slice(bestL, bestL + bestLen);
}

const res = shortestSubstring("figehaeci", new Set(["a", "e", "i"]));
console.log(res === null ? "null" : `"${res}"`); // "aeci"
