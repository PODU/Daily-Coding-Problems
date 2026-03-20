// Integer division without * / %. Subtract doubled divisor. Time O(log^2 q).
function divide(dividend, divisor) {
  let quotient = 0;
  while (dividend >= divisor) {
    let temp = divisor, multiple = 1;
    while (temp + temp <= dividend) { temp += temp; multiple += multiple; }
    dividend -= temp;
    quotient += multiple;
  }
  return quotient;
}

console.log(divide(43, 5));
console.log(divide(100, 10));
