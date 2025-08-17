// Largest rectangle of 1's: per-row histogram + largest-rectangle-in-histogram via monotonic stack.
// Time O(N*M), Space O(M).

function maximalRectangle(mat) {
  if (mat.length === 0 || mat[0].length === 0) return 0;
  const m = mat[0].length;
  const h = new Array(m).fill(0);
  let best = 0;
  for (const row of mat) {
    for (let j = 0; j < m; j++) h[j] = row[j] ? h[j] + 1 : 0;
    const st = [];
    for (let j = 0; j <= m; j++) {
      const cur = j === m ? 0 : h[j];
      while (st.length && h[st[st.length - 1]] >= cur) {
        const height = h[st.pop()];
        const width = st.length === 0 ? j : j - st[st.length - 1] - 1;
        best = Math.max(best, height * width);
      }
      st.push(j);
    }
  }
  return best;
}

const mat = [[1, 0, 0, 0], [1, 0, 1, 1], [1, 0, 1, 1], [0, 1, 0, 0]];
console.log(maximalRectangle(mat));
