// Flood fill via BFS from start pixel; recolor only cells equal to original color.
// Guard against infinite loop when new color == original. Time O(rows*cols), space O(rows*cols).

function floodFill(img, sr, sc, color) {
  const rows = img.length, cols = img[0].length;
  const orig = img[sr][sc];
  if (orig === color) return img;
  const q = [[sr, sc]];
  img[sr][sc] = color;
  const dirs = [[-1, 0], [1, 0], [0, -1], [0, 1]];
  while (q.length) {
    const [r, c] = q.shift();
    for (const [dr, dc] of dirs) {
      const nr = r + dr, nc = c + dc;
      if (nr >= 0 && nr < rows && nc >= 0 && nc < cols && img[nr][nc] === orig) {
        img[nr][nc] = color;
        q.push([nr, nc]);
      }
    }
  }
  return img;
}

function main() {
  const img = [
    ['B', 'B', 'W'],
    ['W', 'W', 'W'],
    ['W', 'W', 'W'],
    ['B', 'B', 'B'],
  ];
  floodFill(img, 2, 2, 'G');
  console.log(img.map((row) => row.join(' ')).join('\n'));
}

main();
