// Day 767: Find all start indices in S that are anagrams of W.
// Sliding window of size |W| with a 26-count array. O(|S|) time, O(1) space.
function findAnagrams(s, w) {
  const n = s.length, m = w.length, res = [];
  if (m > n) return res;
  const need = new Array(26).fill(0), win = new Array(26).fill(0);
  const A = 'a'.charCodeAt(0);
  for (const c of w) need[c.charCodeAt(0) - A]++;
  for (let i = 0; i < n; i++) {
    win[s.charCodeAt(i) - A]++;
    if (i >= m) win[s.charCodeAt(i - m) - A]--;
    if (i >= m - 1 && need.every((v, k) => v === win[k])) res.push(i - m + 1);
  }
  return res;
}

console.log(findAnagrams("abxaba", "ab").join(", ")); // 0, 3, 4
