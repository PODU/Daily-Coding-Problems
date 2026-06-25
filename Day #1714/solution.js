// Max words packed: boggle DFS to enumerate placements + backtracking over words. Exponential worst case, small dict.
const DR = [-1, 1, 0, 0];
const DC = [0, 0, -1, 1];

function solve(matrix, dictionary) {
  const n = matrix.length;
  const taken = Array.from({ length: n }, () => new Array(n).fill(false));
  const words = Array.from(dictionary);
  let best = 0;

  function findPlacements(w) {
    const found = new Set(); // keyed by sorted cell list string
    const results = [];
    function dfs(r, c, idx, path) {
      if (r < 0 || r >= n || c < 0 || c >= n) return;
      const cell = r * n + c;
      if (taken[r][c] || path.has(cell) || matrix[r][c] !== w[idx]) return;
      path.add(cell);
      if (idx === w.length - 1) {
        const key = Array.from(path).sort((a, b) => a - b).join(",");
        if (!found.has(key)) {
          found.add(key);
          results.push(new Set(path));
        }
      } else {
        for (let d = 0; d < 4; d++) dfs(r + DR[d], c + DC[d], idx + 1, path);
      }
      path.delete(cell);
    }
    for (let i = 0; i < n; i++)
      for (let j = 0; j < n; j++) dfs(i, j, 0, new Set());
    return results;
  }

  const used = new Set();
  function search() {
    best = Math.max(best, used.size);
    for (const w of words) {
      if (used.has(w)) continue;
      for (const placement of findPlacements(w)) {
        for (const cell of placement) taken[Math.floor(cell / n)][cell % n] = true;
        used.add(w);
        search();
        used.delete(w);
        for (const cell of placement) taken[Math.floor(cell / n)][cell % n] = false;
      }
    }
  }
  search();
  return best;
}

const dictionary = new Set(['eat', 'rain', 'in', 'rat']);
const matrix = [['e', 'a', 'n'], ['t', 't', 'i'], ['a', 'r', 'a']];
console.log(solve(matrix, dictionary));
