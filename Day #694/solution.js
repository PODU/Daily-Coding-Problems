// Day 694: First missing positive integer in O(n) time, O(1) space.
// Approach: cyclic sort - place each value v in [1,n] at index v-1, then scan.
// Time O(n), Space O(1) (in-place).
function firstMissingPositive(a) {
  const n = a.length;
  for (let i = 0; i < n; i++)
    while (a[i] > 0 && a[i] <= n && a[a[i] - 1] !== a[i]) {
      const j = a[i] - 1;
      [a[i], a[j]] = [a[j], a[i]];
    }
  for (let i = 0; i < n; i++) if (a[i] !== i + 1) return i + 1;
  return n + 1;
}

console.log(firstMissingPositive([3, 4, -1, 1])); // 2
console.log(firstMissingPositive([1, 2, 0]));     // 3
