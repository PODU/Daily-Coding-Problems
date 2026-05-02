// Snakes & Ladders: BFS over squares 1..100, dice rolls 1..6, apply jumps. Min turns 1->100.
// Time: O(100*6). Space: O(100).

function minTurns() {
  const jump = new Map([
    [16, 6], [48, 26], [49, 11], [56, 53], [62, 19], [64, 60], [87, 24], [93, 73], [95, 75], [98, 78], // snakes
    [1, 38], [4, 14], [9, 31], [21, 42], [28, 84], [36, 44], [51, 67], [71, 91], [80, 100],            // ladders
  ]);
  const land = (s) => (jump.has(s) ? jump.get(s) : s);

  const start = land(1);
  const dist = new Array(101).fill(-1);
  const q = [start];
  dist[start] = 0;
  let head = 0;
  while (head < q.length) {
    const s = q[head++];
    if (s === 100) return dist[s];
    for (let d = 1; d <= 6; d++) {
      let nxt = s + d;
      if (nxt > 100) continue;
      nxt = land(nxt);
      if (dist[nxt] === -1) {
        dist[nxt] = dist[s] + 1;
        q.push(nxt);
      }
    }
  }
  return -1;
}

console.log(minTurns());
