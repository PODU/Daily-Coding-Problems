// Count ordered (a,b), a+b=M, a^b=N. c=(M-N)/2; valid if c&N==0.
// Count=2^popcount(N), minus 2 if M==N. Time O(1), Space O(1).
function popcount(n) {
  let c = 0;
  while (n > 0) { c += n & 1; n >>>= 1; }
  return c;
}

function countPairs(M, N) {
  const diff = M - N;
  if (diff < 0 || (diff & 1)) return 0;
  const c = diff / 2;
  if (c & N) return 0;
  let count = 1 << popcount(N);
  if (M === N) count -= 2;
  return count < 0 ? 0 : count;
}

console.log(countPairs(10, 4));
