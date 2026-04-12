// Find smallest window to sort: right bound = last i where a[i] < running max; left bound = first j where a[j] > running min from right.
// Time: O(n), Space: O(1).
"use strict";

function findUnsorted(a) {
  const n = a.length;
  let end = -1, mx = -Infinity;
  for (let i = 0; i < n; i++) {
    if (a[i] < mx) end = i;
    else mx = a[i];
  }
  let start = -1, mn = Infinity;
  for (let i = n - 1; i >= 0; i--) {
    if (a[i] > mn) start = i;
    else mn = a[i];
  }
  return [start, end];
}

function main() {
  const a = [3, 7, 5, 6, 9];
  const [start, end] = findUnsorted(a);
  console.log(`(${start}, ${end})`);
}

main();
