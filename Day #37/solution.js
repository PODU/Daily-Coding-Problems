// Power set via bitmask 0..2^n-1, then order subsets by (size, element order).
// O(2^n * n) time, O(2^n * n) space.
function powerSet(s) {
  const n = s.length;
  const subsets = [];
  for (let mask = 0; mask < (1 << n); mask++) {
    const sub = [];
    for (let i = 0; i < n; i++) if (mask & (1 << i)) sub.push(s[i]);
    subsets.push(sub);
  }
  subsets.sort((a, b) => {
    if (a.length !== b.length) return a.length - b.length;
    for (let i = 0; i < a.length; i++) if (a[i] !== b[i]) return a[i] - b[i];
    return 0;
  });
  return subsets;
}

const subsets = powerSet([1, 2, 3]);
const parts = subsets.map((sub) => "{" + sub.join(", ") + "}");
console.log("{" + parts.join(", ") + "}");
