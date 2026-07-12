// Palindrome pairs: map word->index, split each word, match palindromic halves.
// Time O(N*L^2), Space O(N*L).
function isPal(s) {
  for (let l = 0, r = s.length - 1; l < r; l++, r--) {
    if (s[l] !== s[r]) return false;
  }
  return true;
}

function palindromePairs(words) {
  const idx = new Map();
  words.forEach((w, i) => idx.set(w, i));
  const seen = new Set();
  const res = [];
  for (let i = 0; i < words.length; i++) {
    const w = words[i];
    const n = w.length;
    for (let j = 0; j <= n; j++) {
      const prefix = w.slice(0, j), suffix = w.slice(j);
      if (isPal(prefix)) {
        const rs = suffix.split('').reverse().join('');
        if (idx.has(rs) && idx.get(rs) !== i) {
          const key = idx.get(rs) + ',' + i;
          if (!seen.has(key)) { seen.add(key); res.push([idx.get(rs), i]); }
        }
      }
      if (suffix.length && isPal(suffix)) {
        const rp = prefix.split('').reverse().join('');
        if (idx.has(rp) && idx.get(rp) !== i) {
          const key = i + ',' + idx.get(rp);
          if (!seen.has(key)) { seen.add(key); res.push([i, idx.get(rp)]); }
        }
      }
    }
  }
  res.sort((a, b) => (a[0] !== b[0] ? a[0] - b[0] : a[1] - b[1]));
  return res;
}

const words = ["code", "edoc", "da", "d"];
const res = palindromePairs(words);
console.log("[" + res.map(p => `(${p[0]}, ${p[1]})`).join(", ") + "]");
