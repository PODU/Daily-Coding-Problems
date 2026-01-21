// Day 934: First recurring character = first char that has been seen before while scanning.
// Hash set of seen chars; return on first repeat. Time O(n), Space O(min(n, alphabet)).
function firstRecurring(s) {
  const seen = new Set();
  for (const c of s) {
    if (seen.has(c)) return c;
    seen.add(c);
  }
  return null;
}

for (const s of ["acbbac", "abcdef"]) {
  const c = firstRecurring(s);
  console.log(c === null ? "null" : `"${c}"`);
}
// "b"
// null
