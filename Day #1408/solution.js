// Pancake sort: for each shrinking prefix find its max, flip it to the front,
// then flip it into its final position. Only uses reverse(lst, i, j).
// Time O(n^2) comparisons, O(n) flips, Space O(1).

function reverse(lst, i, j) {
  while (i < j) { [lst[i], lst[j]] = [lst[j], lst[i]]; i++; j--; }
}

function pancakeSort(a) {
  for (let n = a.length; n > 1; n--) {
    let mi = 0;
    for (let i = 1; i < n; i++) if (a[i] > a[mi]) mi = i;
    if (mi !== n - 1) {
      reverse(a, 0, mi);
      reverse(a, 0, n - 1);
    }
  }
  return a;
}

console.log(pancakeSort([3, 1, 4, 1, 5, 9, 2, 6]));
