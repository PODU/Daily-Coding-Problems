// Day 88: Integer division using subtraction of shifted divisor (doubling).
// Time O(log^2 q), Space O(1).
function divide(dividend, divisor) {
  let quotient = 0;
  while (dividend >= divisor) {
    let temp = divisor, multiple = 1;
    while (dividend >= temp * 2) { temp += temp; multiple += multiple; }
    dividend -= temp;
    quotient += multiple;
  }
  return quotient;
}

console.log(divide(10, 3)); // 3
console.log(divide(27, 4)); // 6
