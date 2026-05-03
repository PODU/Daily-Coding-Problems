// Pythagorean triplet: square all, sort, for each largest c^2 two-pointer for a^2+b^2.
// Time: O(n^2), Space: O(n).
function hasTriplet(arr) {
  const sq = arr.map((x) => x * x).sort((a, b) => a - b);
  const n = sq.length;
  for (let c = n - 1; c >= 2; c--) {
    let l = 0, r = c - 1;
    while (l < r) {
      const s = sq[l] + sq[r];
      if (s === sq[c]) return true;
      else if (s < sq[c]) l++;
      else r--;
    }
  }
  return false;
}

console.log(hasTriplet([3, 1, 4, 6, 5]) ? "True" : "False");
console.log(hasTriplet([10, 4, 6, 12, 5]) ? "True" : "False");
