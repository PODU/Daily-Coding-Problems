// Day 191: Min intervals to remove = n - max non-overlapping set (touching allowed).
// Greedy by earliest end. Time O(n log n), Space O(1).
function minRemovals(iv) {
  iv = iv.slice().sort((a, b) => a[1] - b[1]);
  let kept = 0, end = -Infinity;
  for (const [s, e] of iv) {
    if (s >= end) {
      kept++;
      end = e;
    }
  }
  return iv.length - kept;
}

console.log(minRemovals([[7, 9], [2, 4], [5, 8]]));
