// Day 194: Segments p_i->q_i cross iff order of p and q disagree. Count = inversions of q
// after sorting pairs by p. Merge-sort inversion count. Time O(n log n), Space O(n).
function mergeCount(v) {
  if (v.length <= 1) return [v, 0];
  const m = v.length >> 1;
  const [left, lc] = mergeCount(v.slice(0, m));
  const [right, rc] = mergeCount(v.slice(m));
  let cnt = lc + rc, i = 0, j = 0;
  const merged = [];
  while (i < left.length && j < right.length) {
    if (left[i] <= right[j]) merged.push(left[i++]);
    else { merged.push(right[j++]); cnt += left.length - i; }
  }
  while (i < left.length) merged.push(left[i++]);
  while (j < right.length) merged.push(right[j++]);
  return [merged, cnt];
}

function countCrossings(p, q) {
  const order = p.map((_, i) => i).sort((a, b) => p[a] - p[b]);
  const qs = order.map((i) => q[i]);
  return mergeCount(qs)[1];
}

console.log(countCrossings([1, 2, 3, 4], [4, 3, 2, 1]));
