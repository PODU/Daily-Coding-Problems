// Sliding window with last-seen index map; advance left past duplicates, track max window length.
// Time: O(n), Space: O(n).
function longestDistinct(a) {
  const last = new Map();
  let best = 0, left = 0;
  for (let r = 0; r < a.length; r++) {
    if (last.has(a[r]) && last.get(a[r]) >= left) left = last.get(a[r]) + 1;
    last.set(a[r], r);
    best = Math.max(best, r - left + 1);
  }
  return best;
}

if (require.main === module) {
  console.log(longestDistinct([5, 1, 3, 5, 2, 3, 4, 1]));
}
