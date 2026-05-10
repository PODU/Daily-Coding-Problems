// Next greater permutation in-place. Standard next_permutation.
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
  for (let l = i + 1, r = n - 1; l < r; l++, r--) {
    [a[l], a[r]] = [a[r], a[l]];
  }
  return a;
}

const fmt = (a) => "[" + a.join(", ") + "]";

for (const c of [[1, 2, 3], [1, 3, 2], [3, 2, 1]]) {
  nextPermutation(c);
  console.log(fmt(c));
}
