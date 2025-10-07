// Sort characters by descending frequency (ties: first-occurrence order).
// Time: O(n log d), Space: O(n).
function frequencySort(s) {
  const cnt = new Map(), first = new Map();
  for (let i = 0; i < s.length; i++) {
    const c = s[i];
    cnt.set(c, (cnt.get(c) || 0) + 1);
    if (!first.has(c)) first.set(c, i);
  }
  const chars = [...cnt.keys()].sort((a, b) =>
    cnt.get(a) !== cnt.get(b) ? cnt.get(b) - cnt.get(a) : first.get(a) - first.get(b)
  );
  return chars.map(c => c.repeat(cnt.get(c))).join("");
}

console.log(frequencySort("tweet"));
