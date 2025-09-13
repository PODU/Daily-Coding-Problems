// Day 267: Is the black king in check? Locate K, then cast rook/queen orthogonal
// rays and bishop/queen diagonal rays (stopping at the first blocker), and test
// knight and pawn attacks. Time O(8x8) = O(1); space O(1).

function inCheck(board) {
  let kr = -1, kc = -1;
  for (let r = 0; r < 8; r++)
    for (let c = 0; c < 8; c++)
      if (board[r][c] === "K") { kr = r; kc = c; }
  if (kr < 0) return false;

  const at = (r, c) =>
    r >= 0 && r < 8 && c >= 0 && c < 8 ? board[r][c] : null;

  for (const [dr, dc] of [[1, 0], [-1, 0], [0, 1], [0, -1]]) {
    let r = kr + dr, c = kc + dc;
    while (at(r, c) === ".") { r += dr; c += dc; }
    const p = at(r, c);
    if (p === "R" || p === "Q") return true;
  }
  for (const [dr, dc] of [[1, 1], [1, -1], [-1, 1], [-1, -1]]) {
    let r = kr + dr, c = kc + dc;
    while (at(r, c) === ".") { r += dr; c += dc; }
    const p = at(r, c);
    if (p === "B" || p === "Q") return true;
  }
  const kn = [[1, 2], [1, -2], [-1, 2], [-1, -2], [2, 1], [2, -1], [-2, 1], [-2, -1]];
  for (const [dr, dc] of kn) if (at(kr + dr, kc + dc) === "N") return true;
  if (at(kr + 1, kc - 1) === "P" || at(kr + 1, kc + 1) === "P") return true;
  for (let dr = -1; dr <= 1; dr++)
    for (let dc = -1; dc <= 1; dc++)
      if ((dr || dc) && at(kr + dr, kc + dc) === "k") return true;
  return false;
}

const board = [
  "...K....",
  "........",
  ".B......",
  "......P.",
  ".......R",
  "..N.....",
  "........",
  ".....Q..",
];
console.log(inCheck(board) ? "True" : "False");
