// Day 219: Connect 4 (7 cols x 6 rows).
// Approach: drop into lowest empty cell; after each move scan 4 directions from it for 4-in-a-row.
// Win check O(1) per move; board O(W*H).
class Connect4 {
  constructor() {
    this.W = 7;
    this.H = 6;
    this.grid = Array.from({ length: this.H }, () => Array(this.W).fill("."));
    this.height = Array(this.W).fill(0);
  }

  drop(col, player) {
    if (col < 0 || col >= this.W || this.height[col] >= this.H) return -1;
    const r = this.height[col]++;
    this.grid[r][col] = player;
    return r;
  }

  wins(r, c, p) {
    const dirs = [[0, 1], [1, 0], [1, 1], [1, -1]];
    for (const [dr, dc] of dirs) {
      let cnt = 1;
      for (const s of [-1, 1]) {
        let rr = r + dr * s, cc = c + dc * s;
        while (rr >= 0 && rr < this.H && cc >= 0 && cc < this.W && this.grid[rr][cc] === p) {
          cnt++; rr += dr * s; cc += dc * s;
        }
      }
      if (cnt >= 4) return true;
    }
    return false;
  }
}

const g = new Connect4();
const moves = [[0, "R"], [1, "B"], [0, "R"], [1, "B"], [0, "R"], [1, "B"], [0, "R"]];
let winner = null;
for (const [col, player] of moves) {
  const row = g.drop(col, player);
  if (g.wins(row, col, player)) { winner = player; break; }
}
console.log(`Player ${winner} wins!`); // R wins vertically in column 0
