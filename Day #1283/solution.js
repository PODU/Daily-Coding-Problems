// Day 1283: For each element, count smaller elements to its right.
// Fenwick (BIT) over compressed values, scanning right-to-left. Time O(n log n), Space O(n).
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
  const n = a.length;
  const sorted = [...new Set(a)].sort((x, y) => x - y);
  const tree = new Array(sorted.length + 1).fill(0);
  const update = (i) => { for (; i < tree.length; i += i & -i) tree[i]++; };
  const query = (i) => { let s = 0; for (; i > 0; i -= i & -i) s += tree[i]; return s; };
  const res = new Array(n);
  for (let i = n - 1; i >= 0; --i) {
    const rank = lowerBound(sorted, a[i]) + 1;
    res[i] = query(rank - 1);
    update(rank);
  }
  return res;
}

console.log(countSmaller([3, 4, 9, 6, 1])); // [ 1, 1, 2, 1, 0 ]
