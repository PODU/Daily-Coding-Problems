// Largest rectangle in a histogram.
// Monotonic increasing stack of bar indices; pop when a lower bar arrives. O(n) time, O(n) space.

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

const heights = [1, 3, 2, 5];
console.log(largestRectangle(heights));
