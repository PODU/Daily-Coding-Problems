// Sieve of Eratosthenes for primes below N; unbounded incremental sieve (map of next composites).
// Sieve: O(N log log N). Generator yields primes indefinitely.

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

function* genPrimes() {
  // Incremental sieve: map next-composite -> prime that generated it.
  const composites = new Map();
  let candidate = 2;
  while (true) {
    if (!composites.has(candidate)) {
      yield candidate;
      composites.set(candidate * candidate, candidate);
    } else {
      const p = composites.get(candidate);
      composites.delete(candidate);
      let nxt = candidate + p;
      while (composites.has(nxt)) nxt += p;
      composites.set(nxt, p);
    }
    candidate++;
  }
}

function firstN(gen, n) {
  const out = [];
  for (const v of gen) {
    out.push(v);
    if (out.length === n) break;
  }
  return out;
}

console.log("Primes below 30: [" + sieve(30).join(", ") + "]");
console.log("First 10 primes: [" + firstN(genPrimes(), 10).join(", ") + "]");
