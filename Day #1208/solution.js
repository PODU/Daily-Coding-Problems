// Day 1208: Fewest perfect squares summing to N.
// Lagrange four-square + Legendre's three-square theorem. Time O(sqrt N), Space O(1).
function isSquare(n) { const r = Math.round(Math.sqrt(n)); return r * r === n; }

function numSquares(n) {
  if (isSquare(n)) return 1;
  for (let a = 1; a * a <= n; a++) if (isSquare(n - a * a)) return 2;
  let m = n;
  while (m % 4 === 0) m /= 4;
  if (m % 8 === 7) return 4;
  return 3;
}

console.log(numSquares(17)); // 2 (16 + 1)
