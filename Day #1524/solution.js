// First missing positive: cyclic sort (index-as-hash), place each x in [1,n] at index x-1.
// Time O(n), Space O(1) extra (in-place).
function firstMissingPositive(arr) {
  const a = arr.slice();
  const n = a.length;
  for (let i = 0; i < n; i++) {
    while (a[i] >= 1 && a[i] <= n && a[a[i] - 1] !== a[i]) {
      const j = a[i] - 1;
      [a[i], a[j]] = [a[j], a[i]];
    }
  }
  for (let i = 0; i < n; i++) if (a[i] !== i + 1) return i + 1;
  return n + 1;
}

console.log(firstMissingPositive([3, 4, -1, 1])); // 2
console.log(firstMissingPositive([1, 2, 0]));     // 3
