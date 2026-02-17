// Count smaller elements to the right via Fenwick tree + coordinate compression.
// Time O(n log n), Space O(n).
function countSmaller(a) {
  const sorted = [...new Set(a)].sort((x, y) => x - y);
  const rank = new Map();
  sorted.forEach((v, i) => rank.set(v, i + 1));
  const tree = new Array(sorted.length + 1).fill(0);
  const update = (i) => { for (; i < tree.length; i += i & -i) tree[i]++; };
  const query = (i) => { let s = 0; for (; i > 0; i -= i & -i) s += tree[i]; return s; };
  const res = new Array(a.length).fill(0);
  for (let i = a.length - 1; i >= 0; i--) {
    const rk = rank.get(a[i]);
    res[i] = query(rk - 1);
    update(rk);
  }
  return res;
}

console.log('[' + countSmaller([3, 4, 9, 6, 1]).join(', ') + ']');
