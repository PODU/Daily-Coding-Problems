// Sieve of Eratosthenes for primes < N (O(N log log N)); plus an incremental
// generator using a Map of next composite multiples. Space: O(N) sieve.
'use strict';

function sieve(n) {
    const isComp = new Array(Math.max(0, n)).fill(false);
    const primes = [];
    for (let i = 2; i < n; i++) {
        if (!isComp[i]) {
            primes.push(i);
            for (let j = i * i; j < n; j += i) isComp[j] = true;
        }
    }
    return primes;
}

// Incremental generator: yields primes one-by-one via a Map of next composites.
function* primeGen() {
    const composites = new Map();
    let current = 1;
    while (true) {
        current++;
        if (!composites.has(current)) {
            composites.set(current * current, current); // prime found
            yield current;
        } else {
            const prime = composites.get(current);
            composites.delete(current);
            let x = current + prime;
            while (composites.has(x)) x += prime;
            composites.set(x, prime);
        }
    }
}

function main() {
    console.log('[' + sieve(100).join(', ') + ']');
    const gen = primeGen();
    const first10 = [];
    for (let i = 0; i < 10; i++) first10.push(gen.next().value);
    console.log('[' + first10.join(', ') + ']');
}

main();
