// Monte Carlo pi: sample (x,y) in unit square via deterministic splitmix64 RNG
// (BigInt), pi ~= 4*inside/total. Fixed seed -> deterministic. Time O(N), Space O(1).

const MASK = (1n << 64n) - 1n;
let state = 42n;

function nextDouble() {
  state = (state + 0x9E3779B97F4A7C15n) & MASK;
  let z = state;
  z = ((z ^ (z >> 30n)) * 0xBF58476D1CE4E5B9n) & MASK;
  z = ((z ^ (z >> 27n)) * 0x94D049BB133111EBn) & MASK;
  z = (z ^ (z >> 31n)) & MASK;
  return Number(z >> 11n) * (1.0 / 9007199254740992.0);
}

function main() {
  const N = 10000000;
  let inside = 0;
  for (let i = 0; i < N; i++) {
    const x = nextDouble();
    const y = nextDouble();
    if (x * x + y * y <= 1.0) inside++;
  }
  const pi = (4.0 * inside) / N;
  console.log("Estimated pi ≈ " + pi.toFixed(3));
}

main();
