// Circular lock min moves via BFS over 1000 states; each state has 6 neighbors (3 wheels x +/-1 mod 10). O(states) time/space.
'use strict';

function minMoves(target, deadends) {
  const dead = new Set(deadends);
  if (dead.has('000') || dead.has(target)) return -1;
  if (target === '000') return 0;
  const visited = new Set(['000']);
  let queue = [['000', 0]];
  while (queue.length) {
    const [cur, d] = queue.shift();
    for (let i = 0; i < 3; i++) {
      for (const delta of [1, 9]) {
        const nd = (Number(cur[i]) + delta) % 10;
        const nx = cur.slice(0, i) + nd + cur.slice(i + 1);
        if (visited.has(nx) || dead.has(nx)) continue;
        if (nx === target) return d + 1;
        visited.add(nx);
        queue.push([nx, d + 1]);
      }
    }
  }
  return -1;
}

function main() {
  const deadends = ['100', '020', '001'];
  console.log(minMoves('333', deadends));
}

main();
