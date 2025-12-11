// Greedy line wrapping: fit max words per line within width k.
// Return null if any single word exceeds k.
// Time: O(n), Space: O(n).

function wrap(s, k) {
  const words = s.split(" ");
  const out = [];
  let line = "";
  for (const word of words) {
    if (word.length > k) return null;
    if (line.length === 0) line = word;
    else if (line.length + 1 + word.length <= k) line += " " + word;
    else { out.push(line); line = word; }
  }
  if (line.length) out.push(line);
  return out;
}

console.log(wrap("the quick brown fox jumps over the lazy dog", 10));
// [ 'the quick', 'brown fox', 'jumps over', 'the lazy', 'dog' ]
