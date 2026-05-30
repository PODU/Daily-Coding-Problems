// Day 1582: Connect 4 on a 7x6 grid with win detection (H/V/diagonal).
// drop() places a disc; win() scans 4 directions from last move. Time: O(1) per move; Space: O(rows*cols).
"use strict";

const COLS = 7, ROWS = 6;

class Connect4 {
  constructor() {
    this.g = Array.from({ length: ROWS }, () => Array(COLS).fill("."));
  }
  drop(col, p) {
    for (let r = ROWS - 1; r >= 0; r--) if (this.g[r][col] === ".") { this.g[r][col] = p; return r; }
    return -1;
  }
  win(r, c) {
    const p = this.g[r][c];
    for (const [dr, dc] of [[0, 1], [1, 0], [1, 1], [1, -1]]) {
      let cnt = 1;
      for (const s of [-1, 1]) {
        let rr = r + s * dr, cc = c + s * dc;
        while (rr >= 0 && rr < ROWS && cc >= 0 && cc < COLS && this.g[rr][cc] === p) {
          cnt++; rr += s * dr; cc += s * dc;
        }
      }
      if (cnt >= 4) return true;
    }
    return false;
  }
  show() { this.g.forEach((row) => console.log(row.join(" "))); }
}

const game = new Connect4();
const moves = [[0, "R"], [1, "B"], [0, "R"], [1, "B"], [0, "R"], [1, "B"], [0, "R"]];
let winner = null;
for (const [col, p] of moves) {
  const r = game.drop(col, p);
  if (game.win(r, col)) { winner = p; break; }
}
game.show();
console.log("Winner:", winner || "none"); // R
