// Day 210: Collatz conjecture - verify reach to 1 and find longest sequence for n <= 1e6.
// Memoized sequence lengths (cache for values <= LIMIT). Time: ~O(LIMIT * avg steps), Space: O(LIMIT).
const LIMIT = 1000000;
const cache = new Int32Array(LIMIT + 1);
cache[1] = 1;

function collatzLen(start) {
  const path = [];
  let m = start;
  while (m > LIMIT || cache[m] === 0) {
    path.push(m);
    m = m % 2 === 0 ? m / 2 : 3 * m + 1;
  }
  let base = cache[m];
  for (let i = path.length - 1; i >= 0; i--) {
    base++;
    if (path[i] <= LIMIT) cache[path[i]] = base;
  }
  return base;
}

console.log("Collatz length of 27:", collatzLen(27)); // 112
let bestN = 1, bestLen = 1;
for (let n = 1; n <= LIMIT; n++) {
  const l = collatzLen(n);
  if (l > bestLen) { bestLen = l; bestN = n; }
}
console.log(`Longest sequence for n <= 1000000: n=${bestN} (length ${bestLen})`); // n=837799 (length 525)
