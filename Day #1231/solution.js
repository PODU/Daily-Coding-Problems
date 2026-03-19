// Monte Carlo pi estimate with a shared 64-bit LCG (BigInt). Time O(n), Space O(1).
const MASK = (1n << 64n) - 1n;
const A = 6364136223846793005n;
const C = 1442695040888963407n;
const DEN = Number(1n << 53n);

function estimatePi(samples, seed = 42n) {
  let x = seed & MASK;
  let inside = 0;
  for (let i = 0; i < samples; i++) {
    x = (A * x + C) & MASK;
    const px = Number(x >> 11n) / DEN;
    x = (A * x + C) & MASK;
    const py = Number(x >> 11n) / DEN;
    if (px * px + py * py <= 1.0) inside++;
  }
  return (4.0 * inside) / samples;
}

console.log(estimatePi(2000000).toFixed(3));
