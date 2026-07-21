// Day 1856: Collatz conjecture test + longest chain under 1,000,000.
// Memoized step counts (cache values <= LIMIT). ~O(LIMIT * avg-chain) time, O(LIMIT) space.

function main() {
  const LIMIT = 1_000_000;
  const steps = new Int32Array(LIMIT + 1); // steps[n] = steps to reach 1 (0 = unknown; steps[1]=0)
  const allReach1 = true;
  let bestN = 1, bestSteps = 0;

  for (let i = 2; i <= LIMIT; i++) {
    let n = i;
    let cnt = 0;
    while (n !== 1 && (n > LIMIT || steps[n] === 0)) {
      n = n % 2 === 0 ? n / 2 : 3 * n + 1;
      cnt++;
    }
    const total = cnt + (n === 1 ? 0 : steps[n]);
    steps[i] = total;
    if (total > bestSteps) { bestSteps = total; bestN = i; }
  }

  console.log(`All sequences for n in [1, ${LIMIT}] reach 1: ${allReach1}`);
  console.log(`Longest sequence under ${LIMIT}: n = ${bestN} with ${bestSteps + 1} terms`);
  // 837799, 525 terms
}

main();
