// Detect a Pythagorean triplet a^2+b^2=c^2 in an array.
// Square + sort, then fix largest as c and two-pointer. Time O(n^2), Space O(1).

function hasTriplet(nums) {
  const sq = nums.map((x) => x * x).sort((a, b) => a - b);
  const n = sq.length;
  for (let c = n - 1; c >= 2; c--) {
    let a = 0;
    let b = c - 1;
    while (a < b) {
      const s = sq[a] + sq[b];
      if (s === sq[c]) return true;
      if (s < sq[c]) a++;
      else b--;
    }
  }
  return false;
}

const nums = [3, 1, 4, 6, 5];
console.log(hasTriplet(nums)); // expected: true
