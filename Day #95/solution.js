// Day 95: Next lexicographic permutation in place. Find rightmost ascent, swap
// with next-larger suffix element, reverse suffix. O(n) time, O(1) space.
function nextPermutation(a) {
  const n = a.length;
  let i = n - 2;
  while (i >= 0 && a[i] >= a[i + 1]) i--;
  if (i >= 0) {
    let j = n - 1;
    while (a[j] <= a[i]) j--;
    [a[i], a[j]] = [a[j], a[i]];
  }
  for (let lo = i + 1, hi = n - 1; lo < hi; lo++, hi--) {
    [a[lo], a[hi]] = [a[hi], a[lo]];
  }
  return a;
}

for (const arr of [[1, 2, 3], [1, 3, 2], [3, 2, 1]]) {
  console.log(nextPermutation(arr.slice()));
}
// [ 1, 3, 2 ] / [ 2, 1, 3 ] / [ 1, 2, 3 ]
