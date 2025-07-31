// Day 57: Greedy word wrap into lines of length <= k. Null if any word > k.
// Time: O(n), Space: O(n).
function wrap(s, k) {
  const out = [];
  let line = "";
  for (const word of s.split(" ")) {
    if (word.length > k) return null;
    if (line === "") line = word;
    else if (line.length + 1 + word.length <= k) line += " " + word;
    else { out.push(line); line = word; }
  }
  if (line) out.push(line);
  return out;
}

const res = wrap("the quick brown fox jumps over the lazy dog", 10);
console.log(res === null ? "null" : JSON.stringify(res));
