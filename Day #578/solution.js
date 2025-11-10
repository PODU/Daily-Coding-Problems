// Bijective char mapping: maintain s1->s2 (consistent) and s2->s1 (injective) maps. Time O(n), Space O(n).
function bijective(s1, s2) {
  if (s1.length !== s2.length) return false;
  const fwd = {}, bwd = {};
  for (let i = 0; i < s1.length; i++) {
    const a = s1[i], b = s2[i];
    if (a in fwd && fwd[a] !== b) return false;
    if (b in bwd && bwd[b] !== a) return false;
    fwd[a] = b;
    bwd[b] = a;
  }
  return true;
}

if (bijective("abc", "bcd"))
  console.log("true (map a to b, b to c, and c to d)");
if (!bijective("foo", "bar"))
  console.log("false (the o cannot map to two characters)");
