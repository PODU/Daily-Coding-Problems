// Integer division without / * %: subtract largest shifted multiple of divisor.
// Bit-shifting. Time O((log n)^2), Space O(1).
function divide(dividend, divisor) {
  let quotient = 0;
  while (dividend >= divisor) {
    let temp = divisor, multiple = 1;
    while (dividend >= temp * 2) {
      temp <<= 1;
      multiple <<= 1;
    }
    dividend -= temp;
    quotient += multiple;
  }
  return quotient;
}

console.log(divide(43, 8));
console.log(divide(100, 9));
