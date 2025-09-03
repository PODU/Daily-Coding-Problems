// Day 211: All occurrences of pattern in string via KMP.
// Approach: Knuth-Morris-Pratt with LPS failure function. Time O(n+m), Space O(m).
function findOccurrences(s, p) {
  const m = p.length, n = s.length;
  const res = [];
  if (m === 0 || m > n) return res;
  const lps = new Array(m).fill(0);
  for (let i = 1, len = 0; i < m;) {
    if (p[i] === p[len]) lps[i++] = ++len;
    else if (len) len = lps[len - 1];
    else lps[i++] = 0;
  }
  for (let i = 0, j = 0; i < n;) {
    if (s[i] === p[j]) {
      i++; j++;
      if (j === m) { res.push(i - m); j = lps[j - 1]; }
    } else if (j) j = lps[j - 1];
    else i++;
  }
  return res;
}

console.log(findOccurrences("abracadabra", "abr"));
