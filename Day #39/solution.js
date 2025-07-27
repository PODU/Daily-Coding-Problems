// Game of Life on infinite board: track live cells in a set, count neighbors via a
// map over live cells + neighbors each step. O(live*9) per step. Print cropped board.
function step(live) {
  const counts = new Map();
  for (const c of live) {
    const [x, y] = c.split(",").map(Number);
    for (let dx = -1; dx <= 1; dx++)
      for (let dy = -1; dy <= 1; dy++)
        if (dx || dy) {
          const key = `${x + dx},${y + dy}`;
          counts.set(key, (counts.get(key) || 0) + 1);
        }
  }
  const next = new Set();
  for (const [key, n] of counts) {
    if (n === 3 || (live.has(key) && n === 2)) next.add(key);
  }
  return next;
}

function printBoard(live) {
  let minx = Infinity, maxx = -Infinity, miny = Infinity, maxy = -Infinity;
  for (const c of live) {
    const [x, y] = c.split(",").map(Number);
    minx = Math.min(minx, x); maxx = Math.max(maxx, x);
    miny = Math.min(miny, y); maxy = Math.max(maxy, y);
  }
  let out = "";
  for (let y = miny; y <= maxy; y++) {
    let row = "";
    for (let x = minx; x <= maxx; x++) row += live.has(`${x},${y}`) ? "*" : ".";
    out += row + "\n";
  }
  process.stdout.write(out);
}

let live = new Set(["0,1", "1,1", "2,1"]);
const steps = 2;
for (let s = 0; s <= steps; s++) {
  console.log(`Step ${s}:`);
  printBoard(live);
  if (s < steps) live = step(live);
}
