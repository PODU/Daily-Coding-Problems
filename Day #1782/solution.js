// Min squared integers summing to n via Lagrange four-square + Legendre three-square theorem.
// O(sqrt(n)) per query (only the i^2+j^2 check loops), O(1) space.
"use strict";

function isPerfectSquare(n) {
  let r = Math.floor(Math.sqrt(n));
  while (r * r < n) r++;
  while (r * r > n) r--;
  return r * r === n;
}

function numSquares(n) {
  if (isPerfectSquare(n)) return 1;
  let m = n;
  while (m % 4 === 0) m /= 4;
  if (m % 8 === 7) return 4;
  for (let i = 1; i * i <= n; i++) {
    if (isPerfectSquare(n - i * i)) return 2;
  }
  return 3;
}

function main() {
  console.log(numSquares(13)); // 2
  console.log(numSquares(27)); // 3
}

main();
