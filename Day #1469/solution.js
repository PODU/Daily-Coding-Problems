// Chess check detection: locate black king, test pawn/knight attacks and ray-cast
// bishop/rook/queen lines blocked by pieces. Time: O(64); Space: O(1) extra.
'use strict';

function inCheck(b) {
  let kr = -1, kc = -1;
  for (let r = 0; r < 8; r++)
    for (let c = 0; c < 8; c++)
      if (b[r][c] === 'K') { kr = r; kc = c; }
  if (kr < 0) return false;

  // White pawns move up; a pawn at (kr+1, kc+-1) attacks the king.
  for (const dc of [-1, 1]) {
    const pr = kr + 1, pc = kc + dc;
    if (pr >= 0 && pr < 8 && pc >= 0 && pc < 8 && b[pr][pc] === 'P') return true;
  }

  const km = [[1,2],[1,-2],[-1,2],[-1,-2],[2,1],[2,-1],[-2,1],[-2,-1]];
  for (const [dr, dc] of km) {
    const r = kr + dr, c = kc + dc;
    if (r >= 0 && r < 8 && c >= 0 && c < 8 && b[r][c] === 'N') return true;
  }

  for (const [dr, dc] of [[1,1],[1,-1],[-1,1],[-1,-1]]) {
    let r = kr + dr, c = kc + dc;
    while (r >= 0 && r < 8 && c >= 0 && c < 8) {
      const p = b[r][c];
      if (p !== '.') { if (p === 'B' || p === 'Q') return true; break; }
      r += dr; c += dc;
    }
  }

  for (const [dr, dc] of [[1,0],[-1,0],[0,1],[0,-1]]) {
    let r = kr + dr, c = kc + dc;
    while (r >= 0 && r < 8 && c >= 0 && c < 8) {
      const p = b[r][c];
      if (p !== '.') { if (p === 'R' || p === 'Q') return true; break; }
      r += dr; c += dc;
    }
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
console.log(inCheck(board) ? "True" : "False");
