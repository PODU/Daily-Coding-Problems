// Day 1585: Sieve of Eratosthenes + incremental infinite prime generator.
// Sieve marks composites up to N; generator yields primes indefinitely via a sieve map.
// Time: O(N log log N) sieve; Space: O(N).
"use strict";

function sieve(n) {
  const comp = new Array(n).fill(false);
  const primes = [];
  for (let i = 2; i < n; i++) {
    if (!comp[i]) {
      primes.push(i);
      for (let j = i * i; j < n; j += i) comp[j] = true;
    }
  }
  return primes;
}

function* primeGenerator() {
  const composites = new Map();
  let n = 2;
  while (true) {
    if (!composites.has(n)) {
      yield n;
      composites.set(n * n, [n]);
    } else {
      for (const p of composites.get(n)) {
        if (!composites.has(n + p)) composites.set(n + p, []);
        composites.get(n + p).push(p);
      }
      composites.delete(n);
    }
    n++;
  }
}

console.log("Primes < 30:", sieve(30).join(" "));
const gen = primeGenerator();
const first10 = [];
for (let i = 0; i < 10; i++) first10.push(gen.next().value);
console.log("First 10 primes:", first10.join(" "));
