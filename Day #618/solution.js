// Pancake sort using only reverse(lst,i,j): flip current max to front, then to its place.
// Time: O(n^2) comparisons, O(n) flips, Space: O(1).
'use strict';

function reverse(lst, i, j) {
  while (i < j) {
    const t = lst[i]; lst[i] = lst[j]; lst[j] = t;
    i++; j--;
  }
}

function pancakeSort(lst) {
  for (let size = lst.length; size > 1; size--) {
    let maxIdx = 0;
    for (let k = 1; k < size; k++) if (lst[k] > lst[maxIdx]) maxIdx = k;
    if (maxIdx !== size - 1) {
      if (maxIdx !== 0) reverse(lst, 0, maxIdx);
      reverse(lst, 0, size - 1);
    }
  }
  return lst;
}

if (require.main === module) {
  const data = [3, 1, 4, 1, 5, 9, 2, 6];
  console.log('[' + pancakeSort(data).join(', ') + ']');
}
