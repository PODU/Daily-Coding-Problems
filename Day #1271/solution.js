// Day 1271: Implement rand5() from rand7() with uniform probability.
// Rejection sampling: redraw rand7 until result <= 5. Expected O(7/5) calls per sample.
function rand7() { return Math.floor(Math.random() * 7) + 1; }

function rand5() {
  let v;
  do { v = rand7(); } while (v > 5); // reject 6,7 -> uniform 1..5
  return v;
}

const trials = 100000;
const count = new Array(6).fill(0);
for (let i = 0; i < trials; i++) count[rand5()]++;
console.log(`Distribution over ${trials} samples (expect ~${Math.floor(trials / 5)} each):`);
for (let v = 1; v <= 5; v++) console.log(`${v}: ${count[v]}`);
