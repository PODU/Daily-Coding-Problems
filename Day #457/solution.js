// Day 457: All start indices in S that are anagrams of W.
// Fixed-size sliding window of char counts with match counter. Time O(|S|).
function anagramIndices(w, s) {
  const m = w.length, n = s.length, res = [];
  if (m === 0 || m > n) return res;
  const need = new Array(256).fill(0), win = new Array(256).fill(0);
  for (const c of w) need[c.charCodeAt(0)]++;
  let matches = 0;
  for (let c = 0; c < 256; c++) if (need[c] === 0) matches++;
  for (let i = 0; i < n; i++) {
    let ch = s.charCodeAt(i);
    if (win[ch] === need[ch]) matches--;
    win[ch]++;
    if (win[ch] === need[ch]) matches++;
    if (i >= m) {
      ch = s.charCodeAt(i - m);
      if (win[ch] === need[ch]) matches--;
      win[ch]--;
      if (win[ch] === need[ch]) matches++;
    }
    if (i >= m - 1 && matches === 256) res.push(i - m + 1);
  }
  return res;
}

console.log(anagramIndices("ab", "abxaba").join(", ")); // 0, 3, 4
