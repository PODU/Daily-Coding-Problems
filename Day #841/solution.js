// Day 841: count positive integer pairs (a,b) with a+b=M and a^b=N.
// Math: a+b = (a^b) + 2*(a&b) => a&b=(M-N)/2; answer = 2^popcount(N) minus zero-cases. O(1).
function countPairs(M, N) {
  const d = M - N;
  if (d < 0 || (d & 1)) return 0;
  const c = d / 2;                 // c = a & b
  if (c & N) return 0;             // AND and XOR bits cannot overlap
  let pc = 0;
  for (let x = N; x; x >>= 1) pc += x & 1;
  let res = 1 << pc;
  if (c === 0) res -= (N !== 0) ? 2 : 1;
  return Math.max(res, 0);
}

console.log(countPairs(4, 2));     // 2
