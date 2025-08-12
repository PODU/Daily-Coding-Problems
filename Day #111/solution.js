// Day 111: Anagram substrings via fixed sliding window of char counts. O(|S|).
function findAnagrams(s, w) {
  const res = [];
  const n = s.length, m = w.length;
  if (m > n) return res;
  const need = new Array(256).fill(0);
  const win = new Array(256).fill(0);
  for (const c of w) need[c.charCodeAt(0)]++;
  const eq = () => need.every((v, i) => v === win[i]);
  for (let i = 0; i < n; i++) {
    win[s.charCodeAt(i)]++;
    if (i >= m) win[s.charCodeAt(i - m)]--;
    if (i >= m - 1 && eq()) res.push(i - m + 1);
  }
  return res;
}

console.log(findAnagrams("abxaba", "ab").join(", "));
