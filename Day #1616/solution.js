// Goldbach pair: sieve up to n, scan a from 2; first a where a and n-a prime.
// Smallest a => lexicographically smallest [a,b]. Time O(n log log n), Space O(n).
function goldbach(n) {
  const isPrime = new Array(n + 1).fill(true);
  isPrime[0] = isPrime[1] = false;
  for (let i = 2; i * i <= n; i++)
    if (isPrime[i])
      for (let j = i * i; j <= n; j += i) isPrime[j] = false;
  for (let a = 2; a <= n - a; a++)
    if (isPrime[a] && isPrime[n - a]) return [a, n - a];
  return [-1, -1];
}

const n = 4;
const [a, b] = goldbach(n);
console.log(`${a} + ${b} = ${n}`);
