// Reorganize string: greedily place the most frequent remaining char that differs from the last.
// Sorted bucket approach. Time: O(n log A), Space: O(A).
function reorganize(s) {
  const cnt = {};
  for (const c of s) cnt[c] = (cnt[c] || 0) + 1;
  // simple max-heap via array kept sorted by count each step (A is small: <=26)
  const heap = Object.entries(cnt).map(([ch, c]) => [c, ch]);
  const res = [];
  let prev = null;
  while (heap.length) {
    // highest count first; ties broken by smallest char for determinism
    heap.sort((a, b) => b[0] - a[0] || (a[1] < b[1] ? -1 : 1));
    const cur = heap.shift();
    res.push(cur[1]);
    if (prev && prev[0] > 0) heap.push(prev);
    cur[0]--;
    prev = cur;
  }
  return res.length === s.length ? res.join("") : null;
}

console.log(reorganize("aaabbc")); // ababac (a valid arrangement)
console.log(reorganize("aaab"));   // null
