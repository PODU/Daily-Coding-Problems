// Estimate pi via Monte Carlo: fraction of random points in [0,1]^2 inside the
// unit quarter-circle approximates pi/4. Time O(samples), Space O(1).
function mulberry32(a) {
  return function () {
    a |= 0;
    a = (a + 0x6d2b79f5) | 0;
    let t = Math.imul(a ^ (a >>> 15), 1 | a);
    t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t;
    return ((t ^ (t >>> 14)) >>> 0) / 4294967296;
  };
}

function estimatePi(samples, seed = 42) {
  const rng = mulberry32(seed);
  let inside = 0;
  for (let i = 0; i < samples; i++) {
    const x = rng(), y = rng();
    if (x * x + y * y <= 1.0) inside++;
  }
  return (4.0 * inside) / samples;
}

console.log(estimatePi(10000000).toFixed(3));
