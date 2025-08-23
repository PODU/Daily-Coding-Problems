// Day 159: First recurring character. Scan left to right tracking seen chars in
// a set; return the first already seen. Time O(n), Space O(alphabet).
'use strict';

function firstRecurring(s) {
  const seen = new Set();
  for (const c of s) {
    if (seen.has(c)) return c;
    seen.add(c);
  }
  return null;
}

console.log(firstRecurring('acbbac') ?? 'null'); // b
console.log(firstRecurring('abcdef') ?? 'null'); // null
