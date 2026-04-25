// Day 1420: wrap words into lines of length <= k, greedily packing max words/line.
// Approach: greedy single pass over words. O(n) time, O(n) space. null if a word > k.

function wrap(s, k) {
  const out = [];
  let line = "";
  for (const word of s.split(" ")) {
    if (word.length > k) return null; // impossible
    if (line === "") line = word;
    else if (line.length + 1 + word.length <= k) line += " " + word;
    else {
      out.push(line);
      line = word;
    }
  }
  if (line !== "") out.push(line);
  return out;
}

console.log(wrap("the quick brown fox jumps over the lazy dog", 10));
// [ 'the quick', 'brown fox', 'jumps over', 'the lazy', 'dog' ]
