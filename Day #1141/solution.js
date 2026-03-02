// Day 1141: Min cost to pack people (remove gaps).
// Transform p_i -> p_i - i, answer = sum |q_i - median(q)|. Time O(n log n), Space O(m).
function minCost(seats) {
  const q = [];
  let idx = 0;
  for (let i = 0; i < seats.length; i++) if (seats[i]) q.push(i - idx++);
  if (q.length === 0) return 0;
  q.sort((a, b) => a - b);
  const med = q[Math.floor(q.length / 2)];
  return q.reduce((s, v) => s + Math.abs(v - med), 0);
}

console.log(minCost([0, 1, 1, 0, 1, 0, 0, 0, 1])); // 5
