// Day 1302: First missing positive integer in O(n) time, O(1) extra space.
// Cyclic placement: put value v at index v-1; first index i with a[i]!=i+1 gives answer.
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
