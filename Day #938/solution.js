// Day 938: Min moves on a 3-wheel lock from 000 to target, avoiding dead ends. BFS over
// 1000 states, each with 6 neighbors. Time O(1000*6), Space O(1000). Returns null if blocked.
function minMoves(target, deadList) {
  const dead = new Set(deadList);
  const start = "000";
  if (dead.has(start) || dead.has(target)) return null;
  if (start === target) return 0;
  const visited = new Set([start]);
  let queue = [start];
  let depth = 0;
  while (queue.length) {
    const next = [];
    depth++;
    for (const cur of queue) {
      for (let i = 0; i < 3; i++) {
        for (const delta of [1, 9]) {
          const d = (Number(cur[i]) + delta) % 10;
          const nx = cur.slice(0, i) + d + cur.slice(i + 1);
          if (dead.has(nx) || visited.has(nx)) continue;
          if (nx === target) return depth;
          visited.add(nx);
          next.push(nx);
        }
      }
    }
    queue = next;
  }
  return null;
}

console.log(minMoves("123", [])); // 6
const dead = ["100", "900", "010", "090", "001", "009"]; // seal off 000
console.log(minMoves("111", dead)); // null
