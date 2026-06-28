// Day 1730: Fast integer exponentiation (exponentiation by squaring).
// Square the base and halve the exponent each step. Time: O(log y). Space: O(1).

function fastPow(x, y) {
  if (y < 0) {
    // x^(-y) = 1 / x^y; integer result only for x == 1 or -1.
    const inv = fastPow(x, -y);
    return inv === 0 ? 0 : Math.trunc(1 / inv);
  }
  let result = 1;
  let base = x;
  while (y > 0) {
    if (y & 1) result *= base;
    base *= base;
    y >>= 1;
  }
  return result;
}

function main() {
  console.log(fastPow(2, 10)); // 1024
}

main();
