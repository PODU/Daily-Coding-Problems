// Day 634: Largest rectangle in a histogram.
// Approach: monotonic increasing stack of bar indices; pop to settle areas.
// Time: O(N), Space: O(N).
function largestRectangle(h) {
  const st = [];
  let best = 0;
  const n = h.length;
  for (let i = 0; i <= n; i++) {
    const cur = i === n ? 0 : h[i];
    while (st.length && h[st[st.length - 1]] >= cur) {
      const height = h[st.pop()];
      const left = st.length ? st[st.length - 1] : -1;
      const width = i - left - 1;
      best = Math.max(best, height * width);
    }
    st.push(i);
  }
  return best;
}

console.log(largestRectangle([1, 3, 2, 5])); // 6
