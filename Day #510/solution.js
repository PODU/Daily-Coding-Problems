// Day 510: All index pairs (i,j) where words[i]+words[j] is a palindrome.
// Hash map of reversed words + prefix/suffix palindrome checks. Time O(N*L^2).
function isPal(s) {
  let l = 0, r = s.length - 1;
  while (l < r) if (s[l++] !== s[r--]) return false;
  return true;
}

function palindromePairs(words) {
  const rev = new Map();
  words.forEach((w, i) => rev.set(w.split("").reverse().join(""), i));
  const seen = new Set();
  const result = [];
  for (let i = 0; i < words.length; i++) {
    const w = words[i];
    const n = w.length;
    for (let cut = 0; cut <= n; cut++) {
      if (isPal(w.slice(0, cut))) {
        const j = rev.get(w.slice(cut));
        if (j !== undefined && j !== i) {
          const key = j + "," + i;
          if (!seen.has(key)) { seen.add(key); result.push([j, i]); }
        }
      }
      if (cut !== n && isPal(w.slice(cut))) {
        const j = rev.get(w.slice(0, cut));
        if (j !== undefined && j !== i) {
          const key = i + "," + j;
          if (!seen.has(key)) { seen.add(key); result.push([i, j]); }
        }
      }
    }
  }
  result.sort((a, b) => (a[0] - b[0]) || (a[1] - b[1]));
  return result;
}

const words = ["code", "edoc", "da", "d"];
const pairs = palindromePairs(words);
console.log("[" + pairs.map(p => `(${p[0]}, ${p[1]})`).join(", ") + "]");
