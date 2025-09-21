// Day 309: Min movement to pack people with no gaps. Map p_i - i; cost minimized
// at the median of those values. O(M log M).
function minCost(seats) {
  const positions = [];
  for (let i = 0; i < seats.length; i++) if (seats[i] === 1) positions.push(i);
  if (positions.length === 0) return 0;
  const b = positions.map((p, i) => p - i).sort((x, y) => x - y);
  const med = b[b.length >> 1];
  return b.reduce((s, x) => s + Math.abs(x - med), 0);
}
console.log(minCost([0, 1, 1, 0, 1, 0, 0, 0, 1])); // 5
