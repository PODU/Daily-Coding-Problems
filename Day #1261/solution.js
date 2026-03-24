// Day 1261: Palindrome pairs.
// Hashmap of words + prefix/suffix palindrome splits. O(n*k^2) time, O(n*k) space.
function isPal(s, i, j) {
  while (i < j) { if (s[i] !== s[j]) return false; i++; j--; }
  return true;
}

function palindromePairs(words) {
  const idx = new Map();
  words.forEach((w, i) => idx.set(w, i));
  const res = [];
  for (let i = 0; i < words.length; i++) {
    const w = words[i], n = w.length;
    for (let j = 0; j <= n; j++) {
      if (isPal(w, 0, j - 1)) {
        const back = w.slice(j).split('').reverse().join('');
        if (idx.has(back) && idx.get(back) !== i) res.push([idx.get(back), i]);
      }
      if (j !== n && isPal(w, j, n - 1)) {
        const back = w.slice(0, j).split('').reverse().join('');
        if (idx.has(back) && idx.get(back) !== i) res.push([i, idx.get(back)]);
      }
    }
  }
  const seen = new Set();
  const out = [];
  for (const [a, b] of res.sort((p, q) => p[0] - q[0] || p[1] - q[1])) {
    const k = a + ',' + b;
    if (!seen.has(k)) { seen.add(k); out.push([a, b]); }
  }
  return out;
}

const res = palindromePairs(["code", "edoc", "da", "d"]);
console.log("[" + res.map(([a, b]) => `(${a}, ${b})`).join(", ") + "]");
