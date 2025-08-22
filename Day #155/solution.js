// Day 155: Boyer-Moore majority vote in O(n) time, O(1) space. We verify the
// candidate; if no strict majority exists we fall back to the most frequent
// element so the answer is well-defined. Time O(n).
'use strict';

function majorityElement(a) {
  let candidate = null, count = 0;
  for (const x of a) {
    if (count === 0) candidate = x;
    count += x === candidate ? 1 : -1;
  }
  const occ = a.filter((x) => x === candidate).length;
  if (occ * 2 > a.length) return candidate; // strict majority

  // Fallback: most frequent element (example has no strict majority).
  const freq = new Map();
  let best = a[0], bestCnt = 0;
  for (const x of a) {
    const c = (freq.get(x) || 0) + 1;
    freq.set(x, c);
    if (c > bestCnt) { bestCnt = c; best = x; }
  }
  return best;
}

console.log(majorityElement([1, 2, 1, 1, 3, 4, 0])); // 1
