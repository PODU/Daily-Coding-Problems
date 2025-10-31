// a+b = (a^b) + 2*(a&b). So c=(M-N)/2 must be a valid carry disjoint from N.
// Ordered pairs = 2^popcount(N), minus the two zero-cases when c==0. O(log M).
function countPairs(M, N) {
  M = BigInt(M); N = BigInt(N);
  if (M < N || ((M - N) & 1n) === 1n) return 0n;
  const c = (M - N) / 2n;
  if ((c & N) !== 0n) return 0n;
  let bits = 0n, t = N;
  while (t > 0n) { bits += t & 1n; t >>= 1n; }
  let ways = 1n << bits;
  if (c === 0n) ways -= 2n;
  return ways < 0n ? 0n : ways;
}

console.log(countPairs(10, 4).toString()); // 2
