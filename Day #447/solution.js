// Day 447: Integer pow via exponentiation by squaring. O(log y) time, O(1) space.

function ipow(x, y) {
  if (y < 0) return 1 / ipow(x, -y);
  let result = 1, base = x;
  while (y > 0) {
    if (y & 1) result *= base;
    base *= base;
    y >>= 1;
  }
  return result;
}

console.log(ipow(2, 10)); // 1024
console.log(ipow(3, 5));  // 243
console.log(ipow(2, -2)); // 0.25
