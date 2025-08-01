// Fast (binary) exponentiation: square-and-multiply. Time O(log y), Space O(1).
function fastPow(x, y) {
  if (y < 0) return 1 / fastPow(x, -y);
  let result = 1;
  while (y > 0) {
    if (y & 1) result *= x;
    x *= x;
    y >>= 1;
  }
  return result;
}

console.log(fastPow(2, 10)); // 1024
