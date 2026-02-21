// Day 1110: Detect a Pythagorean triplet a^2+b^2=c^2 in an array.
// Square all, sort; for each largest c use two-pointer scan. Time: O(N^2). Space: O(N).
function hasTriplet(arr) {
  const sq = arr.map((x) => x * x).sort((a, b) => a - b);
  const n = sq.length;
  for (let c = n - 1; c >= 2; c--) {
    let l = 0, r = c - 1;
    while (l < r) {
      const s = sq[l] + sq[r];
      if (s === sq[c]) return true;
      if (s < sq[c]) l++; else r--;
    }
  }
  return false;
}

console.log(hasTriplet([3, 1, 4, 6, 5]));   // true (3,4,5)
console.log(hasTriplet([10, 4, 6, 12, 5])); // false
