// Integer division without '/': repeated doubling/subtraction.
// Time: O(log n), Space: O(1).
function divide(a, b) {
  const neg = (a < 0) !== (b < 0);
  let x = Math.abs(a), y = Math.abs(b), q = 0;
  while (x >= y) {
    let temp = y, mult = 1;
    while (x >= temp * 2) { temp *= 2; mult *= 2; }
    x -= temp; q += mult;
  }
  return [neg ? -q : q, x];
}

const [quot, rem] = divide(10, 3);
console.log(`(${quot}, ${rem})`);
