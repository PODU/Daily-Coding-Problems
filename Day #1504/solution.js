// KMP string matching. Returns first occurrence index, or false if not found.
// Time O(N+k), Space O(k).
function kmp(text, pat) {
  const n = text.length, k = pat.length;
  if (k === 0) return 0;
  const lps = new Array(k).fill(0);
  let len = 0, i = 1;
  while (i < k) {
    if (pat[i] === pat[len]) lps[i++] = ++len;
    else if (len) len = lps[len - 1];
    else lps[i++] = 0;
  }
  let j = 0;
  i = 0;
  while (i < n) {
    if (text[i] === pat[j]) {
      i++; j++;
      if (j === k) return i - j;
    } else if (j) j = lps[j - 1];
    else i++;
  }
  return false;
}

console.log(kmp("abxabcabcaby", "abcaby"));
