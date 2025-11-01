// Boggle solver: build a trie from the dictionary, DFS each cell over 8 neighbors
// (each cell used once per path), collect words present in the trie.
// Time: O(cells * 8^L) bounded by trie depth; Space: O(total dictionary chars).

function solve(board, dictionary) {
  const trie = {};
  for (const w of dictionary) {
    let node = trie;
    for (const ch of w) {
      if (!node[ch]) node[ch] = {};
      node = node[ch];
    }
    node['#'] = true;
  }

  const R = board.length, C = board[0].length;
  const found = new Set();

  function dfs(r, c, node, cur, used) {
    const ch = board[r][c];
    if (!node[ch]) return;
    const nxt = node[ch];
    cur += ch;
    if (nxt['#']) found.add(cur);
    used.add(r * C + c);
    for (let dr = -1; dr <= 1; dr++) {
      for (let dc = -1; dc <= 1; dc++) {
        if (dr === 0 && dc === 0) continue;
        const nr = r + dr, nc = c + dc;
        if (nr >= 0 && nr < R && nc >= 0 && nc < C && !used.has(nr * C + nc))
          dfs(nr, nc, nxt, cur, used);
      }
    }
    used.delete(r * C + c);
  }

  for (let r = 0; r < R; r++)
    for (let c = 0; c < C; c++)
      dfs(r, c, trie, "", new Set());
  return [...found].sort();
}

const board = [['o', 'a', 'a', 'n'],
               ['e', 't', 'a', 'e'],
               ['i', 'h', 'k', 'r'],
               ['i', 'f', 'l', 'v']];
const dictionary = ["oath", "pea", "eat", "rain"];
console.log("[" + solve(board, dictionary).join(", ") + "]");
