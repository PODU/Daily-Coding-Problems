// Day 282: Detect Pythagorean triplet. Square + sort, then two-pointer per c.
// Time O(N^2), Space O(N) for squares.
function hasTriplet(arr) {
  const a = arr.map((x) => x * x).sort((p, q) => p - q);
  const n = a.length;
  for (let c = n - 1; c >= 2; c--) {
    let lo = 0, hi = c - 1;
    while (lo < hi) {
      const s = a[lo] + a[hi];
      if (s === a[c]) return true;
      else if (s < a[c]) lo++;
      else hi--;
    }
  }
  return false;
}

console.log(hasTriplet([3, 1, 4, 6, 5]));    // true (3,4,5)
console.log(hasTriplet([10, 4, 6, 12, 5]));  // false
