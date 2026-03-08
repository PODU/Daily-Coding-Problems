// Day 1178: Collatz sequences. Verify each reaches 1 and find the longest start <= 1e6.
// Iterative memoization caching every value along each chain. Time ~O(LIMIT), Space O(LIMIT).

const LIMIT = 1000000;

function main() {
    const memo = new Int32Array(LIMIT + 1);
    memo[1] = 1;

    const seq = [];
    let n = 6;
    while (true) {
        seq.push(n);
        if (n === 1) break;
        n = n % 2 === 0 ? n / 2 : 3 * n + 1;
    }
    console.log(seq.join(" -> "));

    for (let start = 2; start <= LIMIT; start++) {
        if (memo[start]) continue;
        const path = [];
        let m = start;
        while (m > LIMIT || memo[m] === 0) {
            path.push(m);
            m = m % 2 === 0 ? m / 2 : 3 * m + 1;
        }
        let base = memo[m];
        for (let k = path.length - 1; k >= 0; k--) {
            base += 1;
            if (path[k] <= LIMIT) memo[path[k]] = base;
        }
    }

    let bestN = 1, bestLen = 1;
    for (let i = 2; i <= LIMIT; i++) if (memo[i] > bestLen) { bestLen = memo[i]; bestN = i; }
    console.log(`All sequences up to ${LIMIT} reach 1: true`);
    console.log(`Longest sequence for n <= ${LIMIT}: n = ${bestN} (length ${bestLen})`);
}

main();
