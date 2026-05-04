// Least squares summing to n via Legendre/Lagrange: 1 if square, 4 if 4^a(8b+7),
// 2 if i^2+j^2, else 3. Time: O(sqrt(n)); Space: O(1).
'use strict';

function isSquare(x) {
  const r = Math.round(Math.sqrt(x));
  return r * r === x;
}

function numSquares(n) {
  if (isSquare(n)) return 1;
  let m = n;
  while (m % 4 === 0) m /= 4;
  if (m % 8 === 7) return 4;
  for (let i = 1; i * i <= n; i++)
    if (isSquare(n - i * i)) return 2;
  return 3;
}

console.log(numSquares(13));
console.log(numSquares(27));
