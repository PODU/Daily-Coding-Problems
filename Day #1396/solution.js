// KMP substring search: build LPS array, then single scan.
// Time: O(N + k), Space: O(k).

function kmpSearch(s, pat) {
  const N = s.length, k = pat.length;
  if (k === 0) return 0;
  const lps = new Array(k).fill(0);
  for (let i = 1, len = 0; i < k; ) {
    if (pat[i] === pat[len]) lps[i++] = ++len;
    else if (len) len = lps[len - 1];
    else lps[i++] = 0;
  }
  for (let i = 0, j = 0; i < N; ) {
    if (s[i] === pat[j]) { i++; j++; if (j === k) return i - k; }
    else if (j) j = lps[j - 1];
    else i++;
  }
  return false;
}

const res = kmpSearch("abracadabra", "cad");
console.log(res === false ? "False" : res);
