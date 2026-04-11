// Day 1342: Pack the maximum number of dictionary words onto a letter grid (vertex-disjoint adjacency paths).
// Enumerate each word's placements (cell bitmasks) via DFS, then backtrack to select max disjoint set. Exponential worst case.
function maxWords(dictionary, board) {
  const N = board.length, M = board[0].length;
  const dirs = [[-1, 0], [1, 0], [0, -1], [0, 1]];

  function placementsFor(word) {
    const masks = new Set();
    function dfs(idx, r, c, mask) {
      mask |= 1 << (r * M + c);
      if (idx === word.length - 1) { masks.add(mask); return; }
      for (const [dr, dc] of dirs) {
        const nr = r + dr, nc = c + dc;
        if (nr < 0 || nr >= N || nc < 0 || nc >= M) continue;
        if ((mask >> (nr * M + nc)) & 1) continue;
        if (board[nr][nc] !== word[idx + 1]) continue;
        dfs(idx + 1, nr, nc, mask);
      }
    }
    for (let r = 0; r < N; r++)
      for (let c = 0; c < M; c++)
        if (board[r][c] === word[0]) dfs(0, r, c, 0);
    return [...masks];
  }

  const placements = [];
  for (const w of dictionary) {
    const p = placementsFor(w);
    if (p.length) placements.push(p);
  }

  let best = 0;
  function backtrack(i, used, count) {
    if (count + (placements.length - i) <= best) return;
    if (i === placements.length) { best = Math.max(best, count); return; }
    backtrack(i + 1, used, count);
    for (const m of placements[i])
      if (!(m & used)) backtrack(i + 1, used | m, count + 1);
  }
  backtrack(0, 0, 0);
  return best;
}

const dictionary = ["eat", "rain", "in", "rat"];
const matrix = [["e", "a", "n"], ["t", "t", "i"], ["a", "r", "a"]];
console.log(maxWords(dictionary, matrix));
