// Day 771: One-to-one (bijective) character mapping between s1 and s2.
// Track forward and reverse maps; reject conflicts. O(n) time, O(1) space.
function isOneToOne(s1, s2) {
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

console.log(isOneToOne("abc", "bcd")); // true
console.log(isOneToOne("foo", "bar")); // false
