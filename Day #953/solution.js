// Day 953: largest sum of non-adjacent numbers (may pick none -> >= 0).
// incl/excl DP. Time O(n), Space O(1).

function maxNonAdjacent(a) {
  let incl = 0, excl = 0;
  for (const x of a) {
    const ni = excl + x;
    const ne = Math.max(incl, excl);
    incl = ni; excl = ne;
  }
  return Math.max(incl, excl);
}

console.log(maxNonAdjacent([2, 4, 6, 2, 5])); // 13
console.log(maxNonAdjacent([5, 1, 1, 5]));    // 10
