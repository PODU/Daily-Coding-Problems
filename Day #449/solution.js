// Day 449: Goldbach pair. Scan a from 2 upward; the first a where a and n-a are
// both prime gives the lexicographically smallest pair. O(n*sqrt(n)) worst case.

function isPrime(x) {
  if (x < 2) return false;
  for (let i = 2; i * i <= x; i++) if (x % i === 0) return false;
  return true;
}

function goldbach(n) {
  for (let a = 2; a <= n / 2; a++)
    if (isPrime(a) && isPrime(n - a)) return [a, n - a];
  return null;
}

const n = 4;
const [a, b] = goldbach(n);
console.log(`${a} + ${b} = ${n}`); // 2 + 2 = 4
