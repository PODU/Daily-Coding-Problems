// Reverse words in a space-delimited string.
// Approach: reverse whole char array, then reverse each word in place.
// Time: O(n), Space: O(n) (JS strings immutable, operate on array).

function reverseRange(s, i, j) {
  while (i < j) { const t = s[i]; s[i] = s[j]; s[j] = t; i++; j--; }
}

function reverseWords(str) {
  const s = str.split('');
  const n = s.length;
  reverseRange(s, 0, n - 1);
  let start = 0;
  for (let i = 0; i <= n; i++) {
    if (i === n || s[i] === ' ') {
      reverseRange(s, start, i - 1);
      start = i + 1;
    }
  }
  return s.join('');
}

console.log('"' + reverseWords("hello world here") + '"'); // "here world hello"
