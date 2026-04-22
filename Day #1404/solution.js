// KMP: build LPS, scan once, record every full match start. Time O(N+k), Space O(k).

function findAll(s, pat) {
  const res = [];
  const N = s.length, k = pat.length;
  if (k === 0 || k > N) return res;
  const lps = new Array(k).fill(0);
  for (let i = 1, len = 0; i < k; ) {
    if (pat[i] === pat[len]) lps[i++] = ++len;
    else if (len) len = lps[len - 1];
    else lps[i++] = 0;
  }
  for (let i = 0, j = 0; i < N; ) {
    if (s[i] === pat[j]) {
      i++; j++;
      if (j === k) { res.push(i - k); j = lps[j - 1]; }
    } else if (j) j = lps[j - 1];
    else i++;
  }
  return res;
}

console.log(findAll("abracadabra", "abr")); // [ 0, 7 ]
