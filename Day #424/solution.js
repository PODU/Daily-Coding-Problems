// Day 424: Two unique elements via XOR partition. O(n) time, O(1) space.
// XOR all -> a^b; isolate a low set bit; partition & XOR each group -> a, b.
function twoUnique(a) {
  let x = 0;
  for (const v of a) x ^= v;
  const bit = x & (-x);
  let p = 0, q = 0;
  for (const v of a) {
    if (v & bit) p ^= v;
    else q ^= v;
  }
  return [Math.min(p, q), Math.max(p, q)];
}

const [lo, hi] = twoUnique([2, 4, 6, 8, 10, 2, 6, 10]);
console.log(lo + " and " + hi);
