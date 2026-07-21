// Day 1854: Shortest substring containing all chars in a set.
// Sliding window with a need-counter; expand right, contract left while valid. O(n) time, O(|set|) space.

function shortestSubstring(s, need) {
  const want = new Map();
  for (const c of need) want.set(c, (want.get(c) || 0) + 1);
  const required = want.size;
  const win = new Map();
  let formed = 0, bestLen = Infinity, bestL = 0, l = 0;
  for (let r = 0; r < s.length; r++) {
    const c = s[r];
    if (want.has(c)) {
      win.set(c, (win.get(c) || 0) + 1);
      if (win.get(c) === want.get(c)) formed++;
    }
    while (formed === required) {
      if (r - l + 1 < bestLen) { bestLen = r - l + 1; bestL = l; }
      const lc = s[l++];
      if (want.has(lc)) {
        win.set(lc, win.get(lc) - 1);
        if (win.get(lc) < want.get(lc)) formed--;
      }
    }
  }
  return bestLen === Infinity ? null : s.slice(bestL, bestL + bestLen);
}

const res = shortestSubstring('figehaeci', new Set(['a', 'e', 'i']));
console.log(res === null ? 'null' : res); // aeci
