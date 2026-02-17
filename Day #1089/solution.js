// Sliding window maximum via monotonic decreasing deque of indices. Time O(n), Space O(k).
function maxSlidingWindow(a, k) {
  const dq = []; // indices, values decreasing
  const res = [];
  for (let i = 0; i < a.length; i++) {
    if (dq.length && dq[0] <= i - k) dq.shift();
    while (dq.length && a[dq[dq.length - 1]] <= a[i]) dq.pop();
    dq.push(i);
    if (i >= k - 1) res.push(a[dq[0]]);
  }
  return res;
}

const res = maxSlidingWindow([10, 5, 2, 7, 8, 7], 3);
console.log('[' + res.join(', ') + ']');
