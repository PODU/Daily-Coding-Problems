// Count distinct max-heaps from N distinct values: ways(n)=C(n-1,L)*ways(L)*ways(R).
// L = size of left subtree of a complete binary tree with n nodes. Time: O(n^2). Space: O(n^2).

function leftSubtreeSize(n) {
  const h = Math.floor(Math.log2(n));
  const m = n - ((1 << h) - 1);
  const lastCap = 1 << (h - 1);
  return ((1 << (h - 1)) - 1) + Math.min(m, lastCap);
}

function countHeaps(N) {
  const C = Array.from({ length: N + 1 }, () => new Array(N + 1).fill(0));
  for (let i = 0; i <= N; i++) {
    C[i][0] = 1;
    for (let j = 1; j <= i; j++) C[i][j] = C[i - 1][j - 1] + C[i - 1][j];
  }
  const ways = new Array(N + 1).fill(0);
  ways[0] = 1;
  if (N >= 1) ways[1] = 1;
  for (let n = 2; n <= N; n++) {
    const L = leftSubtreeSize(n);
    const R = n - 1 - L;
    ways[n] = C[n - 1][L] * ways[L] * ways[R];
  }
  return ways[N];
}

console.log(countHeaps(3));
