// Collatz: iterate n->n/2 (even) or 3n+1 (odd) counting steps to 1; verify a range.
// Bonus longest under 1e6 via memoized step counts. Time ~O(LIMIT*avg-chain), space O(LIMIT).
'use strict';

function steps(n) {
  let c = 0;
  while (n !== 1) { n = (n % 2 === 0) ? n / 2 : 3 * n + 1; c++; }
  return c;
}

function main() {
  let allReach = true;
  for (let n = 1; n <= 20; n++) if (steps(n) < 0) allReach = false;
  console.log("Collatz conjecture holds for 1..20: " + allReach);

  const LIMIT = 1000000;
  const dp = new Int32Array(LIMIT + 1);
  let bestN = 1, bestLen = 0;
  for (let i = 2; i <= LIMIT; i++) {
    let n = i, c = 0;
    while (n !== 1 && (n > LIMIT || dp[n] === 0)) {
      n = (n % 2 === 0) ? n / 2 : 3 * n + 1; c++;
    }
    c += (n === 1) ? 0 : dp[n];
    dp[i] = c;
    if (c > bestLen) { bestLen = c; bestN = i; }
  }
  console.log(`Longest under 1000000: n=${bestN} with ${bestLen} steps`);
}

main();
