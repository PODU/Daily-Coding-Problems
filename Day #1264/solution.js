// Day 1264: Largest rectangle of 1's in a binary matrix.
// Per-row histogram + largest-rectangle-in-histogram via monotonic stack. O(N*M) time, O(M) space.
function largestInHistogram(h) {
  let best = 0;
  const st = [];
  for (let i = 0; i <= h.length; i++) {
    const cur = i === h.length ? 0 : h[i];
    while (st.length && h[st[st.length - 1]] >= cur) {
      const height = h[st.pop()];
      const left = st.length ? st[st.length - 1] : -1;
      best = Math.max(best, height * (i - left - 1));
    }
    st.push(i);
  }
  return best;
}

function maximalRectangle(m) {
  if (m.length === 0) return 0;
  const cols = m[0].length;
  const h = new Array(cols).fill(0);
  let best = 0;
  for (const row of m) {
    for (let j = 0; j < cols; j++) h[j] = row[j] ? h[j] + 1 : 0;
    best = Math.max(best, largestInHistogram(h));
  }
  return best;
}

const matrix = [[1, 0, 0, 0], [1, 0, 1, 1], [1, 0, 1, 1], [0, 1, 0, 0]];
console.log(maximalRectangle(matrix));
