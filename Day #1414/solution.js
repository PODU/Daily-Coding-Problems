// Day 1414: does a one-to-one (bijective) char mapping s1 -> s2 exist?
// Approach: enforce a consistent forward map AND injective (no two srcs share a target). O(n).

function canMap(s1, s2) {
  if (s1.length !== s2.length) return false;
  const fwd = new Map(), rev = new Map();
  for (let i = 0; i < s1.length; i++) {
    const a = s1[i], b = s2[i];
    if (fwd.has(a) && fwd.get(a) !== b) return false;
    if (rev.has(b) && rev.get(b) !== a) return false;
    fwd.set(a, b); rev.set(b, a);
  }
  return true;
}

console.log(canMap("abc", "bcd") ? "true" : "false"); // true
console.log(canMap("foo", "bar") ? "true" : "false"); // false
