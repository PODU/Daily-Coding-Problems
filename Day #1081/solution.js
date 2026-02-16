// Flood fill via BFS from the seed pixel. Time O(R*C), Space O(R*C).
function floodFill(img, sr, sc, c) {
  const src = img[sr][sc];
  if (src === c) return;
  const R = img.length, C = img[0].length;
  const q = [[sr, sc]];
  img[sr][sc] = c;
  while (q.length) {
    const [r, co] = q.shift();
    for (const [nr, nc] of [[r + 1, co], [r - 1, co], [r, co + 1], [r, co - 1]]) {
      if (nr >= 0 && nr < R && nc >= 0 && nc < C && img[nr][nc] === src) {
        img[nr][nc] = c;
        q.push([nr, nc]);
      }
    }
  }
}

const img = [['B', 'B', 'W'], ['W', 'W', 'W'], ['W', 'W', 'W'], ['B', 'B', 'B']];
floodFill(img, 2, 2, 'G');
console.log(img.map(r => r.join(' ')).join('\n'));
