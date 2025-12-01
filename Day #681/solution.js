// BFS over squares 1..100; from s try rolls 1..6, apply snake/ladder mapping. Time O(N*6), Space O(N).
"use strict";

function minTurns() {
  const snakes = { 16: 6, 48: 26, 49: 11, 56: 53, 62: 19, 64: 60, 87: 24, 93: 73, 95: 75, 98: 78 };
  const ladders = { 1: 38, 4: 14, 9: 31, 21: 42, 28: 84, 36: 44, 51: 67, 71: 91, 80: 100 };
  const jump = { ...snakes, ...ladders };

  const dist = new Array(101).fill(-1);
  dist[1] = 0;                          // start placed on 1; do NOT apply 1->38 here
  const q = [1];
  let head = 0;
  while (head < q.length) {
    const s = q[head++];
    if (s === 100) return dist[s];
    for (let r = 1; r <= 6; r++) {
      let nxt = s + r;
      if (nxt > 100) continue;
      if (jump[nxt] !== undefined) nxt = jump[nxt];
      if (dist[nxt] === -1) { dist[nxt] = dist[s] + 1; q.push(nxt); }
    }
  }
  return dist[100];
}

console.log("Minimum turns: " + minTurns());
