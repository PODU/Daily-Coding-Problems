// KMP pattern matching: build failure (LPS) array, then scan once collecting all match starts.
// Time: O(N+k), Space: O(k).
function findAll(s, p) {
  const n = s.length, k = p.length;
  if (k === 0 || k > n) return [];
  const lps = new Array(k).fill(0);
  for (let i = 1, len = 0; i < k; ) {
    if (p[i] === p[len]) lps[i++] = ++len;
    else if (len) len = lps[len - 1];
    else lps[i++] = 0;
  }
  const res = [];
  for (let i = 0, j = 0; i < n; ) {
    if (s[i] === p[j]) { i++; j++; if (j === k) { res.push(i - k); j = lps[j - 1]; } }
    else if (j) j = lps[j - 1];
    else i++;
  }
  return res;
}

if (require.main === module) {
  const r = findAll("abracadabra", "abr");
  console.log("[" + r.join(", ") + "]");
}
