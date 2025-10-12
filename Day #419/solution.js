// Day 419: Min steps to reduce N to 1 (decrement, or jump to larger factor).
// BFS over values 1..N. Time: O(N*sqrt(N)), Space: O(N).
function minSteps(N) {
  if (N <= 1) return 0;
  const dist = new Int32Array(N + 1).fill(-1);
  dist[N] = 0;
  const q = [N];
  let head = 0;
  while (head < q.length) {
    const v = q[head++];
    if (v === 1) return dist[1];
    if (v - 1 >= 1 && dist[v - 1] === -1) {
      dist[v - 1] = dist[v] + 1;
      q.push(v - 1);
    }
    for (let a = 2; a * a <= v; a++) {
      if (v % a === 0) {
        const larger = v / a;
        if (dist[larger] === -1) {
          dist[larger] = dist[v] + 1;
          q.push(larger);
        }
      }
    }
  }
  return dist[1];
}

const N = 100;
console.log(`${minSteps(N)}  (route: 100 -> 10 -> 9 -> 3 -> 2 -> 1)`);
