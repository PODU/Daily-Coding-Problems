// Day 843: find all start indices of pattern in string using KMP.
// Build failure table, single scan. O(n+m) time, O(m) space.
function kmpSearch(s, p) {
  const n = s.length, m = p.length;
  if (m === 0 || m > n) return [];
  const lps = new Array(m).fill(0);
  for (let i = 1, len = 0; i < m; ) {
    if (p[i] === p[len]) lps[i++] = ++len;
    else if (len) len = lps[len - 1];
    else lps[i++] = 0;
  }
  const res = [];
  for (let i = 0, j = 0; i < n; ) {
    if (s[i] === p[j]) {
      i++; j++;
      if (j === m) { res.push(i - m); j = lps[j - 1]; }
    } else if (j) j = lps[j - 1];
    else i++;
  }
  return res;
}

console.log(kmpSearch("abracadabra", "abr")); // [ 0, 7 ]
