// Day 772: Boggle solver. Trie of dictionary + DFS from each cell over 8 neighbors,
// marking visited. O(W) to build trie; search bounded by trie/board size.
function solveBoggle(boardRows, dict) {
  const trie = {};
  for (const w of dict) {
    let node = trie;
    for (const ch of w) node = node[ch] || (node[ch] = {});
    node.$ = true;
  }
  const board = boardRows.map(r => r.split(''));
  const R = board.length, C = board[0].length, out = new Set();

  function dfs(r, c, node, path) {
    const ch = board[r][c];
    if (ch === '#' || !node[ch]) return;
    const nxt = node[ch];
    path += ch;
    if (nxt.$) out.add(path);
    board[r][c] = '#';
    for (let dr = -1; dr <= 1; dr++)
      for (let dc = -1; dc <= 1; dc++) {
        if (dr === 0 && dc === 0) continue;
        const nr = r + dr, nc = c + dc;
        if (nr >= 0 && nr < R && nc >= 0 && nc < C) dfs(nr, nc, nxt, path);
      }
    board[r][c] = ch;
  }

  for (let r = 0; r < R; r++)
    for (let c = 0; c < C; c++) dfs(r, c, trie, "");
  return [...out].sort();
}

const board = ["oaan", "etae", "ihkr", "iflv"];
const dict = ["oath", "pea", "eat", "rain"];
console.log(solveBoggle(board, dict).join(" ")); // eat oath
