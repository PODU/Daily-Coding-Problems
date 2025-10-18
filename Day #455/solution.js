// Day 455: Conway's Game of Life on an infinite board.
// Set of live cells; tally neighbours each tick. Time O(live) per step.
function step(live) {
  const cnt = new Map();
  for (const k of live) {
    const [r, c] = k.split(",").map(Number);
    for (let dr = -1; dr <= 1; dr++)
      for (let dc = -1; dc <= 1; dc++)
        if (dr || dc) {
          const nk = `${r + dr},${c + dc}`;
          cnt.set(nk, (cnt.get(nk) || 0) + 1);
        }
  }
  const next = new Set();
  for (const [k, n] of cnt)
    if (n === 3 || (n === 2 && live.has(k))) next.add(k);
  return next;
}

function printBoard(live) {
  if (live.size === 0) { console.log("(empty)"); return; }
  const rs = [], cs = [];
  for (const k of live) {
    const [r, c] = k.split(",").map(Number);
    rs.push(r); cs.push(c);
  }
  const r0 = Math.min(...rs), r1 = Math.max(...rs);
  const c0 = Math.min(...cs), c1 = Math.max(...cs);
  for (let r = r0; r <= r1; r++) {
    let line = "";
    for (let c = c0; c <= c1; c++) line += live.has(`${r},${c}`) ? "*" : ".";
    console.log(line);
  }
}

let live = new Set(["1,0", "1,1", "1,2"]); // horizontal blinker
const steps = 2;
console.log("Step 0:"); printBoard(live);
for (let s = 1; s <= steps; s++) {
  live = step(live);
  console.log(`Step ${s}:`); printBoard(live);
}
