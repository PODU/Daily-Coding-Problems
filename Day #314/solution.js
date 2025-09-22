// Day 314: Min broadcast range = max over listeners of distance to nearest tower.
// Sort towers, binary search each listener. O((N+M) log M).
function minRange(listeners, towers) {
  towers = towers.slice().sort((a, b) => a - b);
  const lowerBound = (arr, x) => {
    let lo = 0, hi = arr.length;
    while (lo < hi) { const mid = (lo + hi) >> 1; if (arr[mid] < x) lo = mid + 1; else hi = mid; }
    return lo;
  };
  let ans = 0;
  for (const L of listeners) {
    const idx = lowerBound(towers, L);
    let best = Infinity;
    if (idx < towers.length) best = Math.min(best, towers[idx] - L);
    if (idx > 0) best = Math.min(best, L - towers[idx - 1]);
    ans = Math.max(ans, best);
  }
  return ans;
}
console.log(minRange([1, 5, 11, 20], [4, 8, 15])); // 5
