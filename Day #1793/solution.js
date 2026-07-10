// Collect positions p_i of people, set q_i = p_i - i, answer = sum |q_i - median(q)|.
// Time O(n), Space O(m).
function minCost(seats) {
  const q = [];
  let i = 0;
  for (let j = 0; j < seats.length; j++) {
    if (seats[j] === 1) q.push(j - i++);
  }
  if (q.length === 0) return 0;
  const med = q[Math.floor(q.length / 2)];
  return q.reduce((acc, v) => acc + Math.abs(v - med), 0);
}

const seats = [0, 1, 1, 0, 1, 0, 0, 0, 1];
console.log(minCost(seats)); // expected 5
