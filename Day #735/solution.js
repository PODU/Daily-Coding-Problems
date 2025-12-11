// Day 735: Find any peak element in O(log N).
// Approach: Binary search - move toward the larger neighbor; a peak must lie that way.
// Time: O(log n), Space: O(1).

function findPeak(a) {
  let lo = 0, hi = a.length - 1;
  while (lo < hi) {
    const mid = (lo + hi) >> 1;
    if (a[mid] < a[mid + 1]) lo = mid + 1;
    else hi = mid;
  }
  return lo;
}

const a = [0, 2, 5, 3, 1];
const i = findPeak(a);
console.log(`Peak element: ${a[i]} (index ${i})`); // 5 (index 2)
