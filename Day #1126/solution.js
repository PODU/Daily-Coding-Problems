// Smallest number of perfect squares summing to N.
// Legendre/Lagrange: 1 if perfect square; 4 if N=4^a(8b+7); 2 if sum of two squares; else 3. O(sqrt(N)) time, O(1) space.
function isSquare(n) {
  const r = Math.round(Math.sqrt(n));
  return r * r === n;
}

function numSquares(n) {
  if (isSquare(n)) return 1;
  let m = n;
  while (m % 4 === 0) m /= 4;
  if (m % 8 === 7) return 4;
  for (let a = 1; a * a <= n; a++) if (isSquare(n - a * a)) return 2;
  return 3;
}

const N = 17;
console.log(numSquares(N));
