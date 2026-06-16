// Day 1673: Min wheel rotations from "000" to target avoiding dead ends.
// BFS over <=1000 states, each with 6 neighbors. Time O(1000), Space O(1000).
function openLock(target, deadends) {
  const dead = new Set(deadends);
  const start = "000";
  if (dead.has(start) || dead.has(target)) return null;
  if (start === target) return 0;
  const dist = new Map([[start, 0]]);
  const q = [start];
  let head = 0;
  while (head < q.length) {
    const cur = q[head++];
    for (let i = 0; i < 3; i++) {
      for (const d of [-1, 1]) {
        const digit = (Number(cur[i]) + d + 10) % 10;
        const nxt = cur.slice(0, i) + digit + cur.slice(i + 1);
        if (dead.has(nxt) || dist.has(nxt)) continue;
        dist.set(nxt, dist.get(cur) + 1);
        if (nxt === target) return dist.get(nxt);
        q.push(nxt);
      }
    }
  }
  return null;
}

console.log(openLock("345", ["333"])); // 12
