// Goldbach: iterate a from 2 upward, return first (a, n-a) both prime (lexicographically smallest).
// Time: O(n*sqrt(n)) worst, Space: O(1).
function isPrime(x) {
  if (x < 2) return false;
  for (let i = 2; i * i <= x; i++) if (x % i === 0) return false;
  return true;
}

function goldbach(n) {
  for (let a = 2; a <= n / 2; a++)
    if (isPrime(a) && isPrime(n - a)) return [a, n - a];
  return [-1, -1];
}

const n = 4;
const [a, b] = goldbach(n);
console.log(`${a} + ${b} = ${n}`);
