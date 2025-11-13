// Day 594: Max number of non-overlapping dictionary words packable on a letter board.
// Approach: for each word enumerate all adjacency placements (DFS) as cell bitmasks,
// then backtracking max set-packing to pick the most pairwise-disjoint placements.
// Time: O(words * board * 4^L) to enumerate + exponential packing on small boards.
function maxWords(board, dictionary) {
  const R = board.length, C = board[0].length;

  function find(word, idx, r, c, mask, out) {
    if (r < 0 || c < 0 || r >= R || c >= C) return;
    const bit = 1 << (r * C + c);
    if (mask & bit || board[r][c] !== word[idx]) return;
    mask |= bit;
    if (idx === word.length - 1) { out.add(mask); return; }
    find(word, idx + 1, r + 1, c, mask, out);
    find(word, idx + 1, r - 1, c, mask, out);
    find(word, idx + 1, r, c + 1, mask, out);
    find(word, idx + 1, r, c - 1, mask, out);
  }

  const placements = [];
  for (const w of dictionary) {
    const masks = new Set();
    for (let r = 0; r < R; r++)
      for (let c = 0; c < C; c++) find(w, 0, r, c, 0, masks);
    placements.push(...masks);
  }

  let best = 0;
  function pack(i, used, cnt) {
    if (cnt > best) best = cnt;
    for (let j = i; j < placements.length; j++) {
      if (!(placements[j] & used)) pack(j + 1, used | placements[j], cnt + 1);
    }
  }
  pack(0, 0, 0);
  return best;
}

const board = [["e", "a", "n"], ["t", "t", "i"], ["a", "r", "a"]];
const dictionary = ["eat", "rain", "in", "rat"];
console.log(maxWords(board, dictionary)); // 3
