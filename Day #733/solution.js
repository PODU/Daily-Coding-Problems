// Day 733: Connect 4 on a 7x6 grid.
// Approach: 2D board + per-column heights; after each drop scan 4 directions from the
// placed disc for 4-in-a-row. Time: O(1) per move, Space: O(rows*cols).

class Connect4 {
  constructor() {
    this.COLS = 7; this.ROWS = 6;
    this.board = Array.from({ length: this.ROWS }, () => Array(this.COLS).fill('.'));
    this.height = Array(this.COLS).fill(0);
  }

  drop(col, color) {
    const r = this.height[col]++;
    this.board[r][col] = color;
    for (const [dr, dc] of [[0, 1], [1, 0], [1, 1], [1, -1]]) {
      let cnt = 1;
      for (const s of [-1, 1])
        for (let k = 1; k < 4; k++) {
          const nr = r + dr * k * s, nc = col + dc * k * s;
          if (nr >= 0 && nr < this.ROWS && nc >= 0 && nc < this.COLS && this.board[nr][nc] === color) cnt++;
          else break;
        }
      if (cnt >= 4) return true;
    }
    return false;
  }

  show() {
    for (let r = this.ROWS - 1; r >= 0; r--) console.log(this.board[r].join(' '));
  }
}

const game = new Connect4();
const moves = [0, 0, 1, 1, 2, 2, 3];
let color = 'R', won = false;
for (const m of moves) {
  won = game.drop(m, color);
  if (won) break;
  color = color === 'R' ? 'B' : 'R';
}
game.show();
console.log(won ? (color === 'R' ? 'Red wins!' : 'Black wins!') : 'No winner yet');
