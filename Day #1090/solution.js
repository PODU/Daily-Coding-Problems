// Palindrome pairs: hash words; for each split test palindromic remainder + reversed counterpart.
// Time O(n*k^2), Space O(n*k).
function isPal(s, i, j) {
  while (i < j) { if (s[i] !== s[j]) return false; i++; j--; }
  return true;
}

function palindromePairs(words) {
  const d = new Map();
  words.forEach((w, i) => d.set(w, i));
  const set = new Set();
  for (let i = 0; i < words.length; i++) {
    const w = words[i], L = w.length;
    for (let j = 0; j <= L; j++) {
      if (isPal(w, 0, j - 1)) {
        const r = w.slice(j).split('').reverse().join('');
        if (d.has(r) && d.get(r) !== i) set.add(d.get(r) + ',' + i);
      }
      if (j !== L && isPal(w, j, L - 1)) {
        const l = w.slice(0, j).split('').reverse().join('');
        if (d.has(l) && d.get(l) !== i) set.add(i + ',' + d.get(l));
      }
    }
  }
  const pairs = [...set].map(s => s.split(',').map(Number));
  pairs.sort((a, b) => a[0] - b[0] || a[1] - b[1]);
  return pairs;
}

const pairs = palindromePairs(["code", "edoc", "da", "d"]);
console.log('[' + pairs.map(p => `(${p[0]}, ${p[1]})`).join(', ') + ']');
