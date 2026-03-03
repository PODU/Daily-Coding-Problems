// Day 1144: count positive pairs (a,b) with a+b=M, a^b=N.
// a+b = (a^b) + 2*(a&b) => and=(M-N)/2; valid iff M>=N, even, and&N==0.
// Ordered pairs = 2^popcount(N), minus 2 if and==0 (excludes a=0/b=0). O(1).
function popcount(x) {
  let c = 0;
  while (x > 0) { c += x & 1; x = Math.floor(x / 2); }
  return c;
}

function countPairs(M, N) {
  if (M < N || (M - N) % 2 === 1) return 0;
  const aAnd = (M - N) / 2;
  if ((aAnd & N) !== 0) return 0;
  let cnt = 2 ** popcount(N);
  if (aAnd === 0) cnt -= 2;
  return Math.max(cnt, 0);
}

console.log(countPairs(10, 4)); // 2 -> (7,3) and (3,7)
