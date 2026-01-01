// Day 835: Shortest substring containing all chars in a set.
// Sliding-window min-window: expand right, contract left while valid. O(N) time, O(K) space.

function shortestSubstring(s, charset) {
  const need = new Map();
  for (const c of charset) need.set(c, 1);
  const have = new Map();
  const required = need.size;
  let formed = 0, left = 0, bestL = 0, bestLen = Infinity, found = false;
  for (let right = 0; right < s.length; right++) {
    const ch = s[right];
    if (need.has(ch)) {
      have.set(ch, (have.get(ch) || 0) + 1);
      if (have.get(ch) === need.get(ch)) formed++;
    }
    while (formed === required) {
      if (right - left + 1 < bestLen) {
        bestLen = right - left + 1;
        bestL = left;
        found = true;
      }
      const lc = s[left];
      if (need.has(lc)) {
        have.set(lc, have.get(lc) - 1);
        if (have.get(lc) < need.get(lc)) formed--;
      }
      left++;
    }
  }
  return found ? s.substring(bestL, bestL + bestLen) : null;
}

const res = shortestSubstring("figehaeci", new Set(['a', 'e', 'i']));
console.log(res !== null ? res : "null");
