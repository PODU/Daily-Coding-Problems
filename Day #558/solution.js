// Estimate Pi via Monte Carlo: sample random points in unit square, fraction inside
// quarter circle ~ pi/4. O(S) for S samples. Deterministic LCG seed for reproducibility.
function estimatePi(samples) {
  // simple deterministic LCG to keep result reproducible
  let seed = 12345 >>> 0;
  const rand = () => {
    seed = (1664525 * seed + 1013904223) >>> 0;
    return seed / 4294967296;
  };
  let inside = 0;
  for (let i = 0; i < samples; i++) {
    const x = rand(), y = rand();
    if (x * x + y * y <= 1.0) inside++;
  }
  return (4.0 * inside) / samples;
}

console.log(estimatePi(20000000).toFixed(3)); // ~3.142
