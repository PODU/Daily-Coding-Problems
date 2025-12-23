// Integer exponentiation by squaring. Time O(log y), Space O(1).
// Negative y handled via reciprocal; demo uses pow(2,10). Uses BigInt for 64-bit safety.

function ipow(x, y) {
  if (y < 0n) return Number(1) / Number(ipow(x, -y));
  let result = 1n;
  let base = x;
  let e = y;
  while (e > 0n) {
    if (e & 1n) result *= base;
    base *= base;
    e >>= 1n;
  }
  return result;
}

console.log(ipow(2n, 10n).toString());
