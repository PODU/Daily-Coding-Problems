// Day 707: Min broadcast range. Sort towers; for each listener binary-search the
// nearest tower, answer is max of those min distances. Time O((N+M)logM).
function lowerBound(arr, x) {
  let lo = 0, hi = arr.length;
  while (lo < hi) {
    const mid = (lo + hi) >> 1;
    if (arr[mid] < x) lo = mid + 1; else hi = mid;
  }
  return lo;
}

function minRange(listeners, towers) {
  towers = towers.slice().sort((a, b) => a - b);
  let ans = 0;
  for (const x of listeners) {
    const i = lowerBound(towers, x);
    let best = Infinity;
    if (i < towers.length) best = Math.min(best, towers[i] - x);
    if (i > 0) best = Math.min(best, x - towers[i - 1]);
    ans = Math.max(ans, best);
  }
  return ans;
}

console.log(minRange([1, 5, 11, 20], [4, 8, 15]));
