// Day 827: Min broadcast range.
// Sort towers; for each listener binary-search nearest tower, take min distance;
// answer = max over listeners. Time O((N+M) log M), Space O(1).
function lowerBound(a, key) {
  let lo = 0, hi = a.length;
  while (lo < hi) {
    const mid = (lo + hi) >> 1;
    if (a[mid] < key) lo = mid + 1; else hi = mid;
  }
  return lo;
}

function minBroadcastRange(listeners, towers) {
  towers = towers.slice().sort((x, y) => x - y);
  let ans = 0;
  for (const l of listeners) {
    let best = Infinity;
    const i = lowerBound(towers, l);
    if (i < towers.length) best = Math.min(best, towers[i] - l);
    if (i > 0) best = Math.min(best, l - towers[i - 1]);
    ans = Math.max(ans, best);
  }
  return ans;
}

console.log(minBroadcastRange([1, 5, 11, 20], [4, 8, 15]));
