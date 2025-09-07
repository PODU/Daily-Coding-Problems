// Snakes and Ladders: BFS over squares 1..100, each die roll (1..6) is one edge; apply jumps.
// Time: O(100 * 6), Space: O(100).
function minTurns() {
  const snakes = {16: 6, 48: 26, 49: 11, 56: 53, 62: 19, 64: 60, 87: 24, 93: 73, 95: 75, 98: 78};
  const ladders = {1: 38, 4: 14, 9: 31, 21: 42, 28: 84, 36: 44, 51: 67, 71: 91, 80: 100};
  const jump = {...snakes, ...ladders};
  const apply = p => jump[p] !== undefined ? jump[p] : p;
  const dist = new Array(101).fill(-1);
  const start = apply(1);
  const q = [start];
  dist[start] = 0;
  while (q.length) {
    const p = q.shift();
    if (p === 100) return dist[p];
    for (let d = 1; d <= 6; d++) {
      let np = p + d;
      if (np > 100) continue;
      np = apply(np);
      if (dist[np] === -1) { dist[np] = dist[p] + 1; q.push(np); }
    }
  }
  return dist[100];
}

console.log("Minimum turns: " + minTurns());
