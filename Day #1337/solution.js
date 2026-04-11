// Sieve of Eratosthenes: mark multiples of each prime from p*p. O(N log log N) time, O(N) space.
// Bonus: an incremental-sieve generator function yielding primes indefinitely.

function sieve(n) {
  const isPrime = new Array(Math.max(n, 2)).fill(true);
  isPrime[0] = isPrime[1] = false;
  for (let p = 2; p * p < n; p++) {
    if (isPrime[p]) {
      for (let m = p * p; m < n; m += p) isPrime[m] = false;
    }
  }
  const primes = [];
  for (let i = 2; i < n; i++) if (isPrime[i]) primes.push(i);
  return primes;
}

function* primeGenerator() {
  const composites = new Map();
  let candidate = 1;
  while (true) {
    candidate++;
    if (!composites.has(candidate)) {
      composites.set(candidate * candidate, [candidate]);
      yield candidate;
    } else {
      for (const p of composites.get(candidate)) {
        if (!composites.has(candidate + p)) composites.set(candidate + p, []);
        composites.get(candidate + p).push(p);
      }
      composites.delete(candidate);
    }
  }
}

function main() {
  console.log(sieve(100).join(" "));

  const gen = primeGenerator();
  const first10 = [];
  for (let i = 0; i < 10; i++) first10.push(gen.next().value);
  console.log(first10.join(" "));
}

main();
