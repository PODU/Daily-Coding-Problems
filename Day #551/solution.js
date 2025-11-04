// Von Neumann fair-coin from biased coin: toss pairs, (0,1)->0, (1,0)->1, else retry.
// Expected O(1) tosses per fair toss; O(1) space.

let rngState = 88172645463325252n;
const MASK = (1n << 64n) - 1n;

function nextUniform() { // xorshift64 -> [0,1)
    let x = rngState;
    x ^= (x << 13n) & MASK;
    x ^= x >> 7n;
    x ^= (x << 17n) & MASK;
    rngState = x & MASK;
    return Number(rngState >> 11n) * (1.0 / 9007199254740992.0);
}

function tossBiased() { return nextUniform() < 0.3 ? 1 : 0; } // p(1)=0.3

function fairToss() {
    while (true) {
        const a = tossBiased();
        const b = tossBiased();
        if (a === 0 && b === 1) return 0;
        if (a === 1 && b === 0) return 1;
    }
}

function main() {
    let heads = 0, tails = 0;
    for (let i = 0; i < 100000; i++) {
        if (fairToss() === 1) heads++; else tails++;
    }
    console.log(`heads: ${heads}, tails: ${tails}`);
}

main();
