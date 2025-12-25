// Day 797: Goldbach - two primes summing to even n, lexicographically smallest.
// Scan a from 2 upward; first prime a with prime (n-a) gives smallest pair.
// Time O(n * sqrt(n)), Space O(1).

function isPrime(x) {
  if (x < 2) return false;
  for (let d = 2; d * d <= x; d++) if (x % d === 0) return false;
  return true;
}

function goldbach(n) {
  for (let a = 2; a <= n / 2; a++)
    if (isPrime(a) && isPrime(n - a)) return [a, n - a];
  return [-1, -1];
}

const n = 4;
const [a, b] = goldbach(n);
console.log(`${a} + ${b} = ${n}`); // 2 + 2 = 4
