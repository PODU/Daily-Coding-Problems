// Approach: Monotonic deque of indices; front always holds the window max. O(n) time, O(k) space.

function maxSlidingWindow(nums, k) {
  const dq = []; // indices, decreasing values
  const res = [];
  for (let i = 0; i < nums.length; i++) {
    if (dq.length && dq[0] <= i - k) dq.shift();
    while (dq.length && nums[dq[dq.length - 1]] <= nums[i]) dq.pop();
    dq.push(i);
    if (i >= k - 1) res.push(nums[dq[0]]);
  }
  return res;
}

const nums = [10, 5, 2, 7, 8, 7];
const k = 3;
console.log("[" + maxSlidingWindow(nums, k).join(", ") + "]");
