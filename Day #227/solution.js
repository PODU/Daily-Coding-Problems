// Boggle solver: build a Trie of the dictionary, DFS from every cell over 8 neighbours.
// Time: O(cells * 8^L) bounded by Trie; Space: O(dictionary size).
function boggle(board, dict) {
  const trie = {};
  for (const w of dict) {
    let node = trie;
    for (const ch of w) node = node[ch] || (node[ch] = {});
    node.$ = w;
  }
  const rows = board.length, cols = board[0].length, found = new Set();

  function dfs(r, c, node) {
    const ch = board[r][c];
    if (!node[ch]) return;
    const nxt = node[ch];
    if (nxt.$) found.add(nxt.$);
    board[r][c] = '#';
    for (let dr = -1; dr <= 1; dr++)
      for (let dc = -1; dc <= 1; dc++) {
        if (dr === 0 && dc === 0) continue;
        const nr = r + dr, nc = c + dc;
        if (nr >= 0 && nr < rows && nc >= 0 && nc < cols && board[nr][nc] !== '#')
          dfs(nr, nc, nxt);
      }
    board[r][c] = ch;
  }

  for (let r = 0; r < rows; r++)
    for (let c = 0; c < cols; c++) dfs(r, c, trie);
  return [...found].sort();
}

const board = [
  ['o', 'a', 'a', 'n'],
  ['e', 't', 'a', 'e'],
  ['i', 'h', 'k', 'r'],
  ['i', 'f', 'l', 'v']];
const dict = ["oath", "pea", "eat", "rain"];
console.log("[" + boggle(board, dict).map(w => `'${w}'`).join(", ") + "]");
