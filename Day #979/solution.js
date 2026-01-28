// Conway's Game of Life on an infinite board: store live cells in a set; each step
// tally neighbor counts only around live cells, then apply the 4 rules.
// Time: O(L) per step (L live cells); Space: O(L). Printing bounds to the live region.
"use strict";

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
          if (dr !== 0 || dc !== 0) {
            const k = `${r + dr},${c + dc}`;
            counts.set(k, (counts.get(k) || 0) + 1);
          }
    }
    const next = new Set();
    for (const [k, cnt] of counts) {
      if (cnt === 3 || (cnt === 2 && this.live.has(k))) next.add(k);
    }
    this.live = next;
  }

  print(stepNo) {
    console.log(`Step ${stepNo}:`);
    if (this.live.size === 0) { console.log("(empty)\n"); return; }
    let minR = Infinity, maxR = -Infinity, minC = Infinity, maxC = -Infinity;
    for (const key of this.live) {
      const [r, c] = key.split(",").map(Number);
      minR = Math.min(minR, r); maxR = Math.max(maxR, r);
      minC = Math.min(minC, c); maxC = Math.max(maxC, c);
    }
    let out = "";
    for (let r = minR; r <= maxR; r++) {
      for (let c = minC; c <= maxC; c++)
        out += this.live.has(`${r},${c}`) ? "*" : ".";
      out += "\n";
    }
    console.log(out);
  }
}

const g = new GameOfLife([[0, 0], [0, 1], [0, 2]]); // horizontal blinker
const steps = 2;
g.print(0);
for (let s = 1; s <= steps; s++) {
  g.step();
  g.print(s);
}
