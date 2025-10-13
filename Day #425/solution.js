// Day 425: Chess check detection. Knight/pawn offsets + sliding rays (rook/bishop/queen)
// with blocking. O(1) board scan from the king's square.
function isCheck(board) {
  const n = 8;
  let kr = -1, kc = -1;
  for (let i = 0; i < n; i++)
    for (let j = 0; j < n; j++)
      if (board[i][j] === 'K') { kr = i; kc = j; }

  const inb = (r, c) => r >= 0 && r < n && c >= 0 && c < n;

  // Knight L-shapes
  for (const [dr, dc] of [[-2,-1],[-2,1],[-1,-2],[-1,2],[1,-2],[1,2],[2,-1],[2,1]]) {
    const r = kr + dr, c = kc + dc;
    if (inb(r, c) && board[r][c] === 'N') return true;
  }

  // White pawn attacks toward smaller row -> pawn sits at (kr+1, kc+-1)
  for (const dc of [-1, 1]) {
    const r = kr + 1, c = kc + dc;
    if (inb(r, c) && board[r][c] === 'P') return true;
  }

  // Rook / Queen orthogonal rays
  for (const [dr, dc] of [[-1,0],[1,0],[0,-1],[0,1]]) {
    let r = kr + dr, c = kc + dc;
    while (inb(r, c)) { const p = board[r][c]; if (p !== '.') { if (p === 'R' || p === 'Q') return true; break; } r += dr; c += dc; }
  }

  // Bishop / Queen diagonal rays
  for (const [dr, dc] of [[-1,-1],[-1,1],[1,-1],[1,1]]) {
    let r = kr + dr, c = kc + dc;
    while (inb(r, c)) { const p = board[r][c]; if (p !== '.') { if (p === 'B' || p === 'Q') return true; break; } r += dr; c += dc; }
  }

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
console.log(isCheck(board) ? "True" : "False");
