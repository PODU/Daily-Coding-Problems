// Day 1031: Snakes & Ladders min turns. BFS over squares 1..100, each turn rolls 1..6,
// applying snake/ladder mapping when a roll lands on a key. Time O(100*6), Space O(100).
function minTurns() {
  const jumps = {
    16: 6, 48: 26, 49: 11, 56: 53, 62: 19, 64: 60, 87: 24, 93: 73, 95: 75, 98: 78,
    1: 38, 4: 14, 9: 31, 21: 42, 28: 84, 36: 44, 51: 67, 71: 91, 80: 100,
  };
  const dist = new Array(101).fill(-1);
  const q = [1];
  dist[1] = 0;
  let head = 0;
  while (head < q.length) {
    const s = q[head++];
    if (s === 100) return dist[s];
    for (let d = 1; d <= 6; d++) {
      let nx = s + d;
      if (nx > 100) continue;
      if (jumps[nx] !== undefined) nx = jumps[nx];
      if (dist[nx] === -1) {
        dist[nx] = dist[s] + 1;
        q.push(nx);
      }
    }
  }
  return -1;
}

console.log(minTurns());
