// Connect 4 on a 7x6 grid. drop() places in lowest empty cell; after each move
// check 4-in-a-row in all 4 directions from the placed cell. Time O(1) per move.

const COLS = 7, ROWS = 6;

class Connect4 {
  constructor() {
    this.grid = Array.from({ length: ROWS }, () => new Array(COLS).fill('.'));
    this.turn = 'R';
  }

  drop(col) {
    if (col < 0 || col >= COLS || this.grid[ROWS - 1][col] !== '.') return 'X';
    let r = 0;
    while (this.grid[r][col] !== '.') r++;
    this.grid[r][col] = this.turn;
    const who = this.turn;
    this.turn = this.turn === 'R' ? 'B' : 'R';
    return this.wins(r, col, who) ? who : ' ';
  }

  wins(r, c, p) {
    const dirs = [[0, 1], [1, 0], [1, 1], [1, -1]];
    for (const [dr, dc] of dirs) {
      let cnt = 1;
      for (const s of [-1, 1])
        for (let k = 1; k < 4; k++) {
          const nr = r + dr * k * s, nc = c + dc * k * s;
          if (nr < 0 || nr >= ROWS || nc < 0 || nc >= COLS || this.grid[nr][nc] !== p) break;
          cnt++;
        }
      if (cnt >= 4) return true;
    }
    return false;
  }

  show() {
    for (let r = ROWS - 1; r >= 0; r--) console.log(this.grid[r].join(' '));
  }
}

const game = new Connect4();
let winner = ' ';
for (const m of [0, 1, 0, 1, 0, 1, 0]) { // Red wins vertically in column 0
  const res = game.drop(m);
  if (res === 'R' || res === 'B') { winner = res; break; }
}
game.show();
console.log(winner !== ' ' ? (winner === 'R' ? "Red" : "Black") + " wins!" : "No winner yet.");
