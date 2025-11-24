// First recurring character: single pass with a hash set, return first char already seen.
// Time O(n), Space O(k).
function firstRecurring(s) {
  const seen = new Set();
  for (const c of s) {
    if (seen.has(c)) return c;
    seen.add(c);
  }
  return null;
}

function run(s) {
  const r = firstRecurring(s);
  console.log(r === null ? "null" : r);
}

run("acbbac");
run("abcdef");
