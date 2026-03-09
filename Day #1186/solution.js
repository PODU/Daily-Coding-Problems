// Count intersecting segment pairs: sort segments by p, then count inversions in q.
// Two segments cross iff their p-order and q-order disagree. O(n log n) time, O(n) space.

function mergeCount(a) {
  if (a.length <= 1) return [a, 0];
  const m = a.length >> 1;
  const [left, cl] = mergeCount(a.slice(0, m));
  const [right, cr] = mergeCount(a.slice(m));
  const merged = [];
  let i = 0, j = 0, c = cl + cr;
  while (i < left.length && j < right.length) {
    if (left[i] <= right[j]) merged.push(left[i++]);
    else { merged.push(right[j++]); c += left.length - i; }
  }
  while (i < left.length) merged.push(left[i++]);
  while (j < right.length) merged.push(right[j++]);
  return [merged, c];
}

function countIntersections(p, q) {
  const order = [...p.keys()].sort((x, y) => p[x] - p[y]);
  const qq = order.map((k) => q[k]);
  return mergeCount(qq)[1];
}

const p = [1, 2, 3], q = [3, 1, 2];
console.log(countIntersections(p, q)); // 2
