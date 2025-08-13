// Day 113: Reverse word order in-place: reverse whole, then each word. O(n) time, O(1) extra.
function reverseWords(str) {
  const s = str.split("");
  const n = s.length;
  const rev = (i, j) => {
    while (i < j) { [s[i], s[j]] = [s[j], s[i]]; i++; j--; }
  };
  rev(0, n - 1);
  let start = 0;
  for (let i = 0; i <= n; i++) {
    if (i === n || s[i] === " ") {
      rev(start, i - 1);
      start = i + 1;
    }
  }
  return s.join("");
}

console.log('"' + reverseWords("hello world here") + '"'); // "here world hello"
