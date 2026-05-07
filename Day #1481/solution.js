// Day 1481: Reverse words while keeping delimiters in their original positions.
// Tokenize into word-runs and delimiter chars, reverse word tokens, re-emit.
// Handles trailing/consecutive delimiters. Time O(N), Space O(N).

function reverseWords(s, delims) {
  const tokens = []; // { word: bool, text }
  let i = 0;
  const n = s.length;
  while (i < n) {
    if (delims.has(s[i])) {
      tokens.push({ word: false, text: s[i] });
      ++i;
    } else {
      let j = i;
      while (j < n && !delims.has(s[j])) ++j;
      tokens.push({ word: true, text: s.slice(i, j) });
      i = j;
    }
  }
  const words = tokens.filter((t) => t.word).map((t) => t.text).reverse();
  let k = 0;
  return tokens.map((t) => (t.word ? words[k++] : t.text)).join("");
}

const d = new Set(["/", ":"]);
console.log(reverseWords("hello/world:here", d));   // here/world:hello
console.log(reverseWords("hello/world:here/", d));  // here/world:hello/
console.log(reverseWords("hello//world:here", d));  // here//world:hello
