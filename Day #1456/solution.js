// Day 1456: First recurring character in a string.
// Approach: scan left-to-right, track seen chars in a Set; first char already
// seen is the answer. Time O(n), Space O(1) (fixed alphabet).
function firstRecurring(s) {
  const seen = new Set();
  for (const c of s) {
    if (seen.has(c)) return c;
    seen.add(c);
  }
  return null;
}

for (const s of ["acbbac", "abcdef"]) {
  const r = firstRecurring(s);
  console.log(r === null ? "null" : `"${r}"`);
}
