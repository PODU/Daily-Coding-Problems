// Day 849: Collatz - verify each n reaches 1; bonus: longest chain for n <= 1,000,000.
// Memoized chain lengths via a typed array. ~O(limit) amortized.
function steps(n) {
  let c = 0;
  while (n !== 1) { n = (n & 1) ? 3 * n + 1 : n / 2; c++; }
  return c;
}

function longestUnder(limit) {
  const cache = new Int32Array(limit + 1);
  cache[1] = 1;
  let bestN = 1, bestLen = 1;
  for (let i = 2; i <= limit; i++) {
    let n = i, len = 0;
    while (n >= i || cache[n] === 0) {
      n = (n % 2 === 1) ? 3 * n + 1 : n / 2;
      len++;
      if (n === 1) break;
    }
    const total = len + (n <= limit ? cache[n] : 1);
    cache[i] = total;
    if (total > bestLen) { bestLen = total; bestN = i; }
  }
  return [bestN, bestLen];
}

console.log(`27 reaches 1 in ${steps(27)} steps`); // 111
const [n, ln] = longestUnder(1000000);
console.log(`Longest chain for n <= 1000000: n = ${n} (length ${ln})`); // 837799
