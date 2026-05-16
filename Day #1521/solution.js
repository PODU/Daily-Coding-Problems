// Find all anagram start indices of W in S.
// Sliding window + 26-letter freq + match counter. Time O(|S|), Space O(1).
function findAnagrams(W, S) {
  const n = S.length, m = W.length;
  const res = [];
  if (m === 0 || m > n) return res;
  const need = new Array(26).fill(0);
  const win = new Array(26).fill(0);
  for (const c of W) need[c.charCodeAt(0) - 97]++;
  let matches = 0;
  for (let i = 0; i < 26; i++) if (need[i] === 0) matches++;
  for (let i = 0; i < n; i++) {
    const add = S.charCodeAt(i) - 97;
    win[add]++;
    if (win[add] === need[add]) matches++;
    else if (win[add] === need[add] + 1) matches--;
    if (i >= m) {
      const rem = S.charCodeAt(i - m) - 97;
      win[rem]--;
      if (win[rem] === need[rem]) matches++;
      else if (win[rem] === need[rem] - 1) matches--;
    }
    if (i >= m - 1 && matches === 26) res.push(i - m + 1);
  }
  return res;
}

const idx = findAnagrams("ab", "abxaba");
console.log(idx.join(", "));
