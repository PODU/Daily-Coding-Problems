// n-th perfect number (digits sum to 10): count up from 1 checking digit sums. Time O(answer * digits), Space O(1).
function digitSum(x) {
  let s = 0;
  while (x > 0) { s += x % 10; x = Math.floor(x / 10); }
  return s;
}

function nthPerfect(n) {
  let count = 0, num = 0;
  while (count < n) {
    num++;
    if (digitSum(num) === 10) count++;
  }
  return num;
}

console.log(nthPerfect(1));
console.log(nthPerfect(2));
