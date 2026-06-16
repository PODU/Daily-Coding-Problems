// Day 1678: Integer division without / * %. Subtract largest shifted multiple of
// divisor each round (doubling). Time O(log(quotient)), Space O(1).
function divide(a, b) {
  let q = 0;
  while (a >= b) {
    let temp = b, mult = 1;
    while (a >= temp << 1) { temp <<= 1; mult <<= 1; }
    a -= temp;
    q += mult;
  }
  return q;
}

console.log(divide(43, 8)); // 5
