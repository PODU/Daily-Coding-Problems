// rand5 from rand7 via rejection sampling: keep rand7 values in 1..5. Expected O(1) calls (7/5).

// Seeded PRNG for reproducible demo.
function mulberry32(seed) {
  return function () {
    seed |= 0; seed = (seed + 0x6D2B79F5) | 0;
    let t = Math.imul(seed ^ (seed >>> 15), 1 | seed);
    t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t;
    return ((t ^ (t >>> 14)) >>> 0) / 4294967296;
  };
}
const rand = mulberry32(12345);

function rand7() {
  return Math.floor(rand() * 7) + 1;
}

function rand5() {
  let x = rand7();
  while (x > 5) x = rand7();
  return x;
}

const counts = new Array(6).fill(0);
const trials = 100000;
for (let i = 0; i < trials; i++) counts[rand5()]++;
console.log(`Distribution over ${trials} samples:`);
for (let v = 1; v <= 5; v++) console.log(`${v}: ${counts[v]}`);
