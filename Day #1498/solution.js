// Day 1498: Conway's Game of Life on an infinite board using a Set of live
// "row,col" cells; per step tally neighbor counts over live cells.
// Time O(L) per step (L = live cells), Space O(L).
class GameOfLife {
  constructor(cells) {
    this.live = new Set(cells.map(([r, c]) => `${r},${c}`));
  }
  step() {
    const counts = new Map();
    for (const key of this.live) {
      const [r, c] = key.split(",").map(Number);
      for (let dr = -1; dr <= 1; dr++)
        for (let dc = -1; dc <= 1; dc++)
          if (dr || dc) {
            const k = `${r + dr},${c + dc}`;
            counts.set(k, (counts.get(k) || 0) + 1);
          }
    }
    const next = new Set();
    for (const [k, n] of counts) {
      const alive = this.live.has(k);
      if (n === 3 || (alive && n === 2)) next.add(k);
    }
    this.live = next;
  }
  render() {
    if (this.live.size === 0) return "(empty)";
    let minR = Infinity, maxR = -Infinity, minC = Infinity, maxC = -Infinity;
    for (const key of this.live) {
      const [r, c] = key.split(",").map(Number);
      minR = Math.min(minR, r); maxR = Math.max(maxR, r);
      minC = Math.min(minC, c); maxC = Math.max(maxC, c);
    }
    const lines = [];
    for (let r = minR; r <= maxR; r++) {
      let row = "";
      for (let c = minC; c <= maxC; c++)
        row += this.live.has(`${r},${c}`) ? "*" : ".";
      lines.push(row);
    }
    return lines.join("\n");
  }
}

const game = new GameOfLife([[0, 1], [1, 1], [2, 1]]);
const steps = 2;
console.log("Step 0:");
console.log(game.render());
for (let s = 1; s <= steps; s++) {
  game.step();
  console.log(`Step ${s}:`);
  console.log(game.render());
}
