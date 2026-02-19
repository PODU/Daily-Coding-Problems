// Day 1102: Min total movement to seat people contiguously (order preserved).
// person i lands at start+i; minimize sum|pos[i]-(start+i)| => shift b[i]=pos[i]-i
// to its median. Time: O(N). Space: O(M).
function minCost(seats) {
  const b = [];
  let p = 0;
  for (let i = 0; i < seats.length; i++) if (seats[i] === 1) b.push(i - p++);
  if (b.length === 0) return 0;
  b.sort((x, y) => x - y);
  const med = b[Math.floor(b.length / 2)];
  return b.reduce((acc, x) => acc + Math.abs(x - med), 0);
}

console.log(minCost([0, 1, 1, 0, 1, 0, 0, 0, 1])); // 5
