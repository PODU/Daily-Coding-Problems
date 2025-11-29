// Day 670: Fewest perfect squares summing to n. Lagrange (answer in {1,2,3,4}) +
// Legendre three-square theorem. Time O(sqrt n), Space O(1).
function isSquare(n) { const r = Math.floor(Math.sqrt(n)); return r * r === n || (r + 1) * (r + 1) === n; }

function numSquares(n) {
  if (isSquare(n)) return 1;
  for (let a = 1; a * a <= n; a++) if (isSquare(n - a * a)) return 2;
  let m = n;
  while (m % 4 === 0) m /= 4;
  if (m % 8 === 7) return 4;
  return 3;
}

console.log(numSquares(13)); // 2
console.log(numSquares(27)); // 3
