// Word break via DP with backpointers: dp[i] reachable, prev[i] start of last word. O(n^2) time, O(n) space.

function wordBreak(s, dict) {
  const n = s.length;
  const prev = new Array(n + 1).fill(-2); // -2 = unreachable
  prev[0] = -1;
  for (let i = 1; i <= n; i++) {
    for (let j = i - 1; j >= 0; j--) { // prefer shortest last word
      if (prev[j] !== -2 && dict.has(s.slice(j, i))) {
        prev[i] = j;
        break;
      }
    }
  }
  if (prev[n] === -2) return null;
  const res = [];
  for (let i = n; i > 0; i = prev[i]) res.unshift(s.slice(prev[i], i));
  return res;
}

const fmt = (r) => (r === null ? "null" : "[" + r.join(", ") + "]");

console.log(fmt(wordBreak("thequickbrownfox", new Set(["quick", "brown", "the", "fox"]))));
console.log(fmt(wordBreak("bedbathandbeyond", new Set(["bed", "bath", "bedbath", "and", "beyond"]))));
