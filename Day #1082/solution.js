// Connect 4 engine with O(1)-per-move win detection (scan 4 directions from last disc).
// Time per move O(1), Space O(R*C).
class Connect4 {
  constructor() {
    this.R = 6; this.C = 7;
    this.g = Array.from({ length: 6 }, () => Array(7).fill('.'));
    this.cur = 'R'; this.over = false; this.winner = '.';
  }
  drop(col) {
    if (col < 0 || col >= this.C || this.over) return -1;
    for (let r = this.R - 1; r >= 0; r--) if (this.g[r][col] === '.') { this.g[r][col] = this.cur; return r; }
    return -1;
  }
  won(r, c) {
    const p = this.g[r][c];
    for (const [dr, dc] of [[0, 1], [1, 0], [1, 1], [1, -1]]) {
      let cnt = 1;
      for (const s of [-1, 1]) {
        let nr = r + dr * s, nc = c + dc * s;
        while (nr >= 0 && nr < this.R && nc >= 0 && nc < this.C && this.g[nr][nc] === p) { cnt++; nr += dr * s; nc += dc * s; }
      }
      if (cnt >= 4) return true;
    }
    return false;
  }
  full() { for (let c = 0; c < this.C; c++) if (this.g[0][c] === '.') return false; return true; }
  play(col) {
    const r = this.drop(col); if (r < 0) return false;
    if (this.won(r, col)) { this.over = true; this.winner = this.cur; }
    else if (this.full()) this.over = true;
    else this.cur = this.cur === 'R' ? 'B' : 'R';
    return true;
  }
  show() { console.log(this.g.map(row => row.join(' ')).join('\n')); }
}

const game = new Connect4();
for (const m of [0, 1, 0, 1, 0, 1, 0]) game.play(m); // R wins vertically in column 0
game.show();
console.log(game.winner !== '.' ? `Player ${game.winner} wins!` : 'Draw');
