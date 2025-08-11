// Day 101: Goldbach. Sieve primes up to n, then the smallest prime a with n-a
// prime gives the lexicographically smallest pair. O(n log log n).
function goldbach(n) {
  const composite = new Uint8Array(n + 1);
  for (let i = 2; i * i <= n; i++)
    if (!composite[i])
      for (let j = i * i; j <= n; j += i) composite[j] = 1;
  for (let a = 2; a <= n / 2; a++)
    if (!composite[a] && !composite[n - a]) return [a, n - a];
  return null;
}

const [a, b] = goldbach(4);
console.log(`${a} + ${b} = ${a + b}`); // 2 + 2 = 4
