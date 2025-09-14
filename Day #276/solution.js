// Day 276: KMP pattern search. Time O(N + k), Space O(k) -- beats O(N*k).
// Returns start index of first match, or false.
function kmp(text, pat) {
  const n = text.length, k = pat.length;
  if (k === 0) return 0;
  const lps = new Array(k).fill(0);
  for (let i = 1, len = 0; i < k;) {
    if (pat[i] === pat[len]) lps[i++] = ++len;
    else if (len) len = lps[len - 1];
    else lps[i++] = 0;
  }
  for (let i = 0, j = 0; i < n;) {
    if (text[i] === pat[j]) {
      i++; j++;
      if (j === k) return i - j;
    } else if (j) j = lps[j - 1];
    else i++;
  }
  return false;
}

// print -1 for the not-found case so every language reports the same
const show = (r) => console.log(r === false ? -1 : r);
show(kmp("abxabcabcaby", "abcaby")); // 6
show(kmp("abxabcabcaby", "zzz"));    // -1
