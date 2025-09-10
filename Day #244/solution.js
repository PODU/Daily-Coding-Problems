// Sieve of Eratosthenes: primes < N in O(N log log N) time, O(N) space.
// Plus an indefinite prime generator (JS generator function) using trial division by known primes.

function sieve(n) {
  const comp = new Uint8Array(Math.max(n, 0));
  const primes = [];
  for (let i = 2; i < n; i++) {
    if (!comp[i]) {
      primes.push(i);
      for (let j = i * i; j < n; j += i) comp[j] = 1;
    }
  }
  return primes;
}

function* primeGen() {
  const primes = [];
  let cand = 1;
  while (true) {
    cand++;
    let isPrime = true;
    for (const p of primes) {
      if (p * p > cand) break;
      if (cand % p === 0) { isPrime = false; break; }
    }
    if (isPrime) { primes.push(cand); yield cand; }
  }
}

function main() {
  console.log(sieve(100).join(" "));

  const g = primeGen();
  const out = [];
  for (let i = 0; i < 10; i++) out.push(g.next().value);
  console.log(out.join(" "));
}

main();
