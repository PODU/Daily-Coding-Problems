// Day 1270: Find all start indices in S that are anagrams of W.
// Fixed-size sliding window with a 26-length count + match counter. O(|S|) time.
function findAnagrams(w, s) {
  const res = [];
  const m = w.length, n = s.length;
  if (m > n) return res;
  const need = new Array(26).fill(0), win = new Array(26).fill(0);
  const a = "a".charCodeAt(0);
  for (const c of w) need[c.charCodeAt(0) - a]++;
  const eq = () => need.every((v, i) => v === win[i]);
  for (let i = 0; i < n; i++) {
    win[s.charCodeAt(i) - a]++;
    if (i >= m) win[s.charCodeAt(i - m) - a]--;
    if (i >= m - 1 && eq()) res.push(i - m + 1);
  }
  return res;
}

console.log(findAnagrams("ab", "abxaba").join(", "));
