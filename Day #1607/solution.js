// Bijective char mapping s1->s2: lengths equal + consistent forward map + injective (no two s1 chars share an s2 char).
// Time O(n), Space O(1) (alphabet-bounded maps).

function bijectiveMap(s1, s2) {
  if (s1.length !== s2.length) return false;
  const fwd = new Map(), rev = new Map();
  for (let i = 0; i < s1.length; i++) {
    const a = s1[i], b = s2[i];
    if (fwd.has(a)) {
      if (fwd.get(a) !== b) return false;
    } else if (rev.has(b)) {
      return false;
    } else {
      fwd.set(a, b);
      rev.set(b, a);
    }
  }
  return true;
}

console.log(bijectiveMap("abc", "bcd"));
console.log(bijectiveMap("foo", "bar"));
