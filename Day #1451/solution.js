// Day 1451: Next lexicographic permutation in place (wraps to smallest).
// Find rightmost ascent, swap with next-larger suffix element, reverse suffix.
// Time O(n), Space O(1).
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
}

function show(a) {
  nextPermutation(a);
  console.log("[" + a.join(",") + "]");
}

show([1, 2, 3]); // [1,3,2]
show([1, 3, 2]); // [2,1,3]
show([3, 2, 1]); // [1,2,3]
