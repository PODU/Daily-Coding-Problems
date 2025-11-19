// Combination lock: BFS over 1000 states from "000" to target, avoiding deadends.
// Each of 3 wheels has 2 neighbors (+1/-1 mod 10) => 6 neighbors. Time/space O(1000).
'use strict';

function openLock(deadends, target, start = '000') {
  const dead = new Set(deadends);
  if (dead.has(start)) return null;
  if (start === target) return 0;
  const visited = new Set([start]);
  let frontier = [start];
  let steps = 0;
  while (frontier.length) {
    steps++;
    const next = [];
    for (const cur of frontier) {
      for (let w = 0; w < 3; w++) {
        for (const d of [1, -1]) {
          const digit = (Number(cur[w]) + d + 10) % 10;
          const nxt = cur.slice(0, w) + digit + cur.slice(w + 1);
          if (dead.has(nxt) || visited.has(nxt)) continue;
          if (nxt === target) return steps;
          visited.add(nxt);
          next.push(nxt);
        }
      }
    }
    frontier = next;
  }
  return null;
}

const r1 = openLock(['010', '021'], '020');
console.log('Min moves to open lock (target 020):', r1);

const r2 = openLock(['001', '010', '100', '009', '090', '900'], '111');
console.log('Impossible case (target 111):', r2 === null ? 'None' : r2);
