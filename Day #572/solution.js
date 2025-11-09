// Next greater permutation in-place (lexicographic). If none, wrap to smallest.
// Approach: find pivot, successor swap, reverse suffix. Time O(n), Space O(1).
"use strict";

function nextPermutation(a) {
  const n = a.length;
  let i = n - 2;
  while (i >= 0 && a[i] >= a[i + 1]) i--;
  if (i >= 0) {
    let j = n - 1;
    while (a[j] <= a[i]) j--;
    [a[i], a[j]] = [a[j], a[i]];
  }
  for (let l = i + 1, r = n - 1; l < r; l++, r--) [a[l], a[r]] = [a[r], a[l]];
  return a;
}

function run(a) {
  console.log("[" + nextPermutation(a).join(",") + "]");
}

run([1, 2, 3]);
run([1, 3, 2]);
run([3, 2, 1]);
