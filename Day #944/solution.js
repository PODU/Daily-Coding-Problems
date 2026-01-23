// Day 944: Next greater permutation of an integer's digits (in-place on digit array).
// Find pivot, swap with next larger from the right, reverse suffix. Time O(d), Space O(d).
function nextPermutation(num) {
  const d = String(num).split("");
  const n = d.length;
  let i = n - 2;
  while (i >= 0 && d[i] >= d[i + 1]) i--;
  if (i < 0) return -1; // already the largest arrangement
  let j = n - 1;
  while (d[j] <= d[i]) j--;
  [d[i], d[j]] = [d[j], d[i]];
  let l = i + 1, r = n - 1;
  while (l < r) { [d[l], d[r]] = [d[r], d[l]]; l++; r--; }
  return Number(d.join(""));
}

console.log(nextPermutation(48975)); // 49578
