// Split array into k contiguous partitions minimizing the max partition sum.
// Binary search on the answer + greedy feasibility. Time: O(n log(sum)). Space: O(1).
function feasible(a, k, cap) {
  let cur = 0, parts = 1;
  for (const x of a) {
    if (x > cap) return false;
    if (cur + x > cap) { parts++; cur = x; }
    else cur += x;
  }
  return parts <= k;
}

function splitArray(a, k) {
  let lo = Math.max(...a), hi = a.reduce((s, x) => s + x, 0);
  while (lo < hi) {
    const mid = Math.floor((lo + hi) / 2);
    if (feasible(a, k, mid)) hi = mid; else lo = mid + 1;
  }
  return lo;
}

console.log(splitArray([5, 1, 2, 7, 3, 4], 3)); // 8
