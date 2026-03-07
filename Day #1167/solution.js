// Fast (binary) exponentiation by squaring using BigInt for a 64-bit-safe result.
// Time: O(log y), Space: O(1).
function fastPow(x, y) {
  let result = 1n;
  let base = BigInt(x);
  let e = BigInt(y);
  while (e > 0n) {
    if (e & 1n) result *= base;
    base *= base;
    e >>= 1n;
  }
  return result;
}

console.log(fastPow(2, 10).toString());
