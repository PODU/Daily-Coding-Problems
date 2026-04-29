// Day 1438: Largest rectangle in a histogram.
// Approach: monotonic increasing stack of bar indices; pop to compute areas.
// Time: O(n), Space: O(n).

function largestRectangle(heights) {
  const st = []; // indices with increasing heights
  let best = 0;
  const n = heights.length;
  for (let i = 0; i <= n; i++) {
    const h = i === n ? 0 : heights[i];
    while (st.length && heights[st[st.length - 1]] >= h) {
      const top = st.pop();
      const width = st.length === 0 ? i : i - st[st.length - 1] - 1;
      best = Math.max(best, heights[top] * width);
    }
    st.push(i);
  }
  return best;
}

console.log(largestRectangle([1, 3, 2, 5])); // 6
