// Day 459: Fewest perfect squares summing to N.
// Lagrange/Legendre theorem -> answer in {1,2,3,4}, O(sqrt N).
// Reconstruct one decomposition guided by the count.
function isSquare(n) {
  if (n < 0) return false;
  const r = Math.round(Math.sqrt(n));
  return r * r === n;
}

function minSquares(n) {
  if (isSquare(n)) return 1;
  let m = n;
  while (m % 4 === 0) m /= 4;
  if (m % 8 === 7) return 4;
  for (let i = 1; i * i <= n; i++) if (isSquare(n - i * i)) return 2;
  return 3;
}

function decompose(n) {
  let k = minSquares(n);
  const res = [];
  while (k > 0) {
    if (k === 1) { res.push(n); break; }
    for (let i = Math.floor(Math.sqrt(n)); i >= 1; i--) {
      if (minSquares(n - i * i) === k - 1) {
        res.push(i * i);
        n -= i * i;
        k--;
        break;
      }
    }
  }
  return res;
}

const n = 17;
console.log(`${minSquares(n)} (${decompose(n).join(" + ")})`); // 2 (16 + 1)
