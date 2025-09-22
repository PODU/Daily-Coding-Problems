// Day 313: Min moves on a 3-wheel lock from 000 to target, avoiding dead ends.
// BFS over <=1000 states. O(1000) time. Returns null when unreachable.
function openLock(target, deadends) {
  const dead = new Set(deadends);
  const start = "000";
  if (dead.has(start)) return null;
  if (start === target) return 0;
  const q = [start];
  const dist = new Map([[start, 0]]);
  while (q.length) {
    const cur = q.shift();
    for (let i = 0; i < 3; i++) for (const d of [-1, 1]) {
      const digit = (Number(cur[i]) + d + 10) % 10;
      const nx = cur.slice(0, i) + digit + cur.slice(i + 1);
      if (dead.has(nx) || dist.has(nx)) continue;
      dist.set(nx, dist.get(cur) + 1);
      if (nx === target) return dist.get(nx);
      q.push(nx);
    }
  }
  return null;
}
console.log(openLock("123", [])); // 6
