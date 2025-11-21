// Day 638: Reverse the words in a string (in-place on a char array).
// Approach: reverse whole array, then reverse each word back.
// Time: O(n), Space: O(1) extra.
function reverseWords(s) {
  const a = s.split("");
  const n = a.length;
  const rev = (i, j) => { while (i < j) { [a[i], a[j]] = [a[j], a[i]]; i++; j--; } };
  rev(0, n - 1);
  let i = 0;
  while (i < n) {
    let j = i;
    while (j < n && a[j] !== " ") j++;
    rev(i, j - 1);
    i = j + 1;
  }
  return a.join("");
}

console.log(reverseWords("hello world here")); // here world hello
