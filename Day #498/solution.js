// Smallest window to sort so the whole array is sorted.
// Two passes: left->right track max for right bound; right->left track min for left bound.
// Time: O(n), Space: O(1).

function windowToSort(a) {
  const n = a.length;
  let left = -1, right = -1;
  let maxSoFar = a[0];
  for (let i = 1; i < n; i++) {
    if (a[i] < maxSoFar) right = i;
    else maxSoFar = a[i];
  }
  let minSoFar = a[n - 1];
  for (let i = n - 2; i >= 0; i--) {
    if (a[i] > minSoFar) left = i;
    else minSoFar = a[i];
  }
  return [left, right];
}

function main() {
  const a = [3, 7, 5, 6, 9];
  const [left, right] = windowToSort(a);
  console.log(`(${left}, ${right})`);
}

main();
