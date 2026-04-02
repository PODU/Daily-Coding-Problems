// Day 1287: Minimum bonuses so each employee out-earns lower-scoring neighbors.
// Two passes (left->right, right->left), take max. Time O(n), Space O(n).
function bonuses(a) {
  const n = a.length;
  const b = new Array(n).fill(1);
  for (let i = 1; i < n; i++) if (a[i] > a[i - 1]) b[i] = b[i - 1] + 1;
  for (let i = n - 2; i >= 0; i--) if (a[i] > a[i + 1]) b[i] = Math.max(b[i], b[i + 1] + 1);
  return b;
}

console.log(bonuses([10, 40, 200, 1000, 60, 30])); // [ 1, 2, 3, 4, 2, 1 ]
