// Day 1474: Boggle solver. Trie of dictionary + DFS from each cell over
// 8-adjacent neighbors (no reuse), pruned by trie prefixes.
// Time O(R*C*8^L) worst case; Space O(total dict chars).

function boggle(board, words) {
  const trie = {};
  for (const w of words) {
    let node = trie;
    for (const ch of w) node = node[ch] || (node[ch] = {});
    node['#'] = w;
  }
  const rows = board.length, cols = board[0].length;
  const found = new Set();

  function dfs(r, c, node) {
    const ch = board[r][c];
    if (ch === '*' || !(ch in node)) return;
    const nxt = node[ch];
    if ('#' in nxt) found.add(nxt['#']);
    board[r][c] = '*';
    for (let dr = -1; dr <= 1; ++dr)
      for (let dc = -1; dc <= 1; ++dc) {
        if (dr === 0 && dc === 0) continue;
        const nr = r + dr, nc = c + dc;
        if (nr >= 0 && nr < rows && nc >= 0 && nc < cols) dfs(nr, nc, nxt);
      }
    board[r][c] = ch;
  }

  for (let r = 0; r < rows; ++r)
    for (let c = 0; c < cols; ++c) dfs(r, c, trie);
  return [...found].sort();
}

const board = [
  ['o', 'a', 'a', 'n'],
  ['e', 't', 'a', 'e'],
  ['i', 'h', 'k', 'r'],
  ['i', 'f', 'l', 'v'],
];
console.log(boggle(board, ["oath", "pea", "eat", "rain"])); // [ 'eat', 'oath' ]
