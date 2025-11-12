// Build histogram heights per row; largest rectangle in histogram via monotonic stack.
// Time O(N*M), Space O(M).
"use strict";

function largestInHistogram(h) {
  const st = [];
  let best = 0;
  const n = h.length;
  for (let i = 0; i <= n; i++) {
    const cur = i === n ? 0 : h[i];
    while (st.length && h[st[st.length - 1]] >= cur) {
      const height = h[st.pop()];
      const left = st.length ? st[st.length - 1] : -1;
      best = Math.max(best, height * (i - left - 1));
    }
    st.push(i);
  }
  return best;
}

function maximalRectangle(mat) {
  if (mat.length === 0) return 0;
  const m = mat[0].length;
  const h = new Array(m).fill(0);
  let best = 0;
  for (const row of mat) {
    for (let j = 0; j < m; j++) h[j] = row[j] ? h[j] + 1 : 0;
    best = Math.max(best, largestInHistogram(h));
  }
  return best;
}

const matrix = [
  [1, 0, 0, 0],
  [1, 0, 1, 1],
  [1, 0, 1, 1],
  [0, 1, 0, 0],
];
console.log(maximalRectangle(matrix));
