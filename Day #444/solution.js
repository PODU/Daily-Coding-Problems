// Day 444: KMP string matching in O(N + k) time, O(k) space (beats O(N*k)).
// Returns the start index of the first match, or false if absent.

function buildLPS(p) {
  const lps = new Array(p.length).fill(0);
  let len = 0, i = 1;
  while (i < p.length) {
    if (p[i] === p[len]) { lps[i++] = ++len; }
    else if (len) { len = lps[len - 1]; }
    else { lps[i++] = 0; }
  }
  return lps;
}

function kmpSearch(text, pat) {
  if (pat.length === 0) return 0;
  const lps = buildLPS(pat);
  let i = 0, j = 0;
  while (i < text.length) {
    if (text[i] === pat[j]) {
      i++; j++;
      if (j === pat.length) return i - j;
    } else if (j) {
      j = lps[j - 1];
    } else {
      i++;
    }
  }
  return false;
}

console.log(kmpSearch("abxabcabcaby", "abcaby")); // 6
