// E[T] = sum_t (1 - (1-p)^n - n*p*(1-p)^(n-1)), p=2^-t (P(>1 coin alive after t rounds)). Sum until negligible. O(iterations) time, O(1) space.
'use strict';

function expectedRounds(n) {
    let total = 0.0;
    for (let t = 0; t < 1000; t++) {
        const p = Math.pow(2.0, -t);
        const q = 1.0 - p;
        const term = 1.0 - Math.pow(q, n) - n * p * Math.pow(q, n - 1);
        total += term;
        if (t > 0 && term < 1e-15) break;
    }
    return total;
}

const n = 4;
console.log(`Expected rounds for n=${n}: ${expectedRounds(n).toFixed(4)}`);
