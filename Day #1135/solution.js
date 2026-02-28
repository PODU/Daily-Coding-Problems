// Power set via bitmask enumeration, sorted by (size, lexicographic). O(2^n * n).
function powerSet(nums) {
  const n = nums.length;
  const subsets = [];
  for (let mask = 0; mask < (1 << n); mask++) {
    const s = [];
    for (let i = 0; i < n; i++) if (mask & (1 << i)) s.push(nums[i]);
    subsets.push(s);
  }
  subsets.sort((a, b) => {
    if (a.length !== b.length) return a.length - b.length;
    for (let i = 0; i < a.length; i++) if (a[i] !== b[i]) return a[i] - b[i];
    return 0;
  });
  return subsets;
}

const subsets = powerSet([1, 2, 3]);
console.log("[" + subsets.map((s) => "[" + s.join(", ") + "]").join(", ") + "]");
