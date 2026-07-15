// Snakes & Ladders minimum turns: BFS over board squares (unweighted shortest path).
// Each square has up to 6 die-roll edges; snakes/ladders redirect the landing square.
// Time: O(100*6). Space: O(100).
function minTurns(jump, size = 100) {
  const dist = new Array(size + 1).fill(-1);
  dist[1] = 0;
  const q = [1]; // begin on square 1; jumps trigger only on rolled squares
  while (q.length) {
    const sq = q.shift();
    if (sq === size) return dist[sq];
    for (let d = 1; d <= 6; d++) {
      let nxt = sq + d;
      if (nxt > size) continue;
      if (jump.has(nxt)) nxt = jump.get(nxt);
      if (dist[nxt] === -1) { dist[nxt] = dist[sq] + 1; q.push(nxt); }
    }
  }
  return dist[size];
}

const jump = new Map([
  [16,6],[48,26],[49,11],[56,53],[62,19],[64,60],[87,24],[93,73],[95,75],[98,78],
  [1,38],[4,14],[9,31],[21,42],[28,84],[36,44],[51,67],[71,91],[80,100],
]);
console.log(minTurns(jump)); // 7
