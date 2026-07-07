// Largest sum of non-adjacent numbers via include/exclude DP; empty selection (0) allowed.
// Time: O(N), Space: O(1).
function maxNonAdjacent(a) {
  let incl = 0, excl = 0;
  for (const n of a) {
    const ni = excl + n;
    const ne = Math.max(incl, excl);
    incl = ni; excl = ne;
  }
  return Math.max(incl, excl);
}

console.log(maxNonAdjacent([2, 4, 6, 2, 5]));
console.log(maxNonAdjacent([5, 1, 1, 5]));
