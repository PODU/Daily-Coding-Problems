// Day 1419: reverse the order of space-delimited words, in-place.
// Approach: reverse whole char array, then reverse each word. O(n) time, O(1) extra space.

function reverseRange(s, i, j) {
  while (i < j) {
    const t = s[i];
    s[i] = s[j];
    s[j] = t;
    i++;
    j--;
  }
}

function reverseWords(s) {
  const n = s.length;
  reverseRange(s, 0, n - 1);
  let start = 0;
  for (let i = 0; i <= n; i++) {
    if (i === n || s[i] === " ") {
      reverseRange(s, start, i - 1);
      start = i + 1;
    }
  }
  return s;
}

const chars = "hello world here".split("");
console.log(reverseWords(chars).join("")); // here world hello
