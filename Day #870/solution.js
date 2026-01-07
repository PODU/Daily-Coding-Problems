// Day 870: Largest rectangle of 1's in a binary matrix.
// Approach: build per-row histogram of consecutive 1's, apply largest-rectangle-in-histogram.
// Time: O(N*M), Space: O(M).
'use strict';

function largestInHist(h) {
  const st = [];
  let best = 0;
  const n = h.length;
  for (let i = 0; i <= n; i++) {
    const cur = i === n ? 0 : h[i];
    while (st.length && h[st[st.length - 1]] >= cur) {
      const height = h[st.pop()];
      const width = st.length === 0 ? i : i - st[st.length - 1] - 1;
      best = Math.max(best, height * width);
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
    best = Math.max(best, largestInHist(h));
  }
  return best;
}

const mat = [
  [1, 0, 0, 0],
  [1, 0, 1, 1],
  [1, 0, 1, 1],
  [0, 1, 0, 0]
];
console.log(maximalRectangle(mat)); // 4
