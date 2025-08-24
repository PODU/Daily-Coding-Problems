// Day 165: Count smaller elements to the right. Coordinate-compress, then sweep
// right to left with a Fenwick tree (BIT). Time O(n log n), Space O(n).
'use strict';

function lowerBound(arr, key) {
  let lo = 0, hi = arr.length;
  while (lo < hi) {
    const mid = (lo + hi) >> 1;
    if (arr[mid] < key) lo = mid + 1;
    else hi = mid;
  }
  return lo;
}

function countSmaller(a) {
  const sortedUnique = [...new Set(a)].sort((x, y) => x - y);
  const tree = new Array(sortedUnique.length + 1).fill(0);
  const update = (i, v) => { for (; i < tree.length; i += i & -i) tree[i] += v; };
  const query = (i) => { let s = 0; for (; i > 0; i -= i & -i) s += tree[i]; return s; };

  const res = new Array(a.length).fill(0);
  for (let i = a.length - 1; i >= 0; i--) {
    const rank = lowerBound(sortedUnique, a[i]) + 1;
    res[i] = query(rank - 1);
    update(rank, 1);
  }
  return res;
}

console.log(JSON.stringify(countSmaller([3, 4, 9, 6, 1]))); // [1,1,2,1,0]
