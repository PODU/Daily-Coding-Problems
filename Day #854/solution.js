// Day 854: greedy word wrap - pack max words per line of length <= k; null if any word > k.
// Single pass over words. O(total characters).
function wrap(s, k) {
  const out = [];
  let line = "";
  for (const word of s.split(" ")) {
    if (word.length > k) return null;
    if (line === "") line = word;
    else if (line.length + 1 + word.length <= k) line += " " + word;
    else { out.push(line); line = word; }
  }
  if (line !== "") out.push(line);
  return out;
}

console.log(wrap("the quick brown fox jumps over the lazy dog", 10));
// [ 'the quick', 'brown fox', 'jumps over', 'the lazy', 'dog' ]
