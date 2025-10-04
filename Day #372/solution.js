// Day 372: Count digits of a natural number without loops.
// Recursion: 1 digit for n<10, else 1 + digits(floor(n/10)). Time O(d), Space O(d).
'use strict';

function numDigits(n) {
  return n < 10 ? 1 : 1 + numDigits(Math.floor(n / 10));
}

console.log(numDigits(0));      // 1
console.log(numDigits(7));      // 1
console.log(numDigits(42));     // 2
console.log(numDigits(12345));  // 5
