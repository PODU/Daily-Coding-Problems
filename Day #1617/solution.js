// Falling dominoes: two-pass force accumulation (R adds +, L adds -, decay between).
// Sign of net force decides L/R/.; equal force stays '.'. Time O(n), Space O(n).
function pushDominoes(d) {
  const n = d.length;
  const force = new Array(n).fill(0);
  let f = 0;
  for (let i = 0; i < n; i++) {
    if (d[i] === 'R') f = n;
    else if (d[i] === 'L') f = 0;
    else f = Math.max(f - 1, 0);
    force[i] += f;
  }
  f = 0;
  for (let i = n - 1; i >= 0; i--) {
    if (d[i] === 'L') f = n;
    else if (d[i] === 'R') f = 0;
    else f = Math.max(f - 1, 0);
    force[i] -= f;
  }
  return force.map(x => (x > 0 ? 'R' : x < 0 ? 'L' : '.')).join('');
}

console.log(pushDominoes(".L.R....L"));
console.log(pushDominoes("..R...L.L"));
