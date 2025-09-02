// Day 206: Apply permutation P to array (result[P[i]] = a[i]).
// In-place via cycle following on the permutation. Time: O(n), Space: O(1).
function applyPermutation(a, perm) {
  const p = [...perm];
  for (let i = 0; i < a.length; i++) {
    while (p[i] !== i) {
      const j = p[i];
      [a[i], a[j]] = [a[j], a[i]];
      p[i] = p[j];
      p[j] = j;
    }
  }
  return a;
}

console.log(applyPermutation(["a", "b", "c"], [2, 1, 0])); // ['c', 'b', 'a']
