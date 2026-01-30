// Day 988: Minimum number of perfect squares summing to n.
// Number theory (Lagrange + Legendre's three-square theorem). O(sqrt n) time, O(1) space.

function isSquare(x) {
  const r = Math.floor(Math.sqrt(x));
  return r * r === x || (r + 1) * (r + 1) === x;
}

function numSquares(n) {
  if (isSquare(n)) return 1;
  let m = n;
  while (m % 4 === 0) m /= 4;        // strip factors of 4
  if (m % 8 === 7) return 4;         // Legendre: 4^a(8b+7) needs 4
  for (let a = 1; a * a <= n; a++)
    if (isSquare(n - a * a)) return 2;
  return 3;
}

console.log("13 ->", numSquares(13)); // expected 2
console.log("27 ->", numSquares(27)); // expected 3
