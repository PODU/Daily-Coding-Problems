// Max sum of non-adjacent numbers: track best-including vs best-excluding current.
// Time: O(n), Space: O(1). (Empty pick allowed, so negatives can be skipped.)
function maxNonAdjacent(nums) {
  let incl = 0, excl = 0;
  for (const n of nums) {
    const ni = excl + n;
    const ne = Math.max(incl, excl);
    incl = ni;
    excl = ne;
  }
  return Math.max(incl, excl);
}

console.log(maxNonAdjacent([2, 4, 6, 2, 5]));
