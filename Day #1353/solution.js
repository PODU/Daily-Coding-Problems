// Count max-heaps: count(n)=C(n-1,L)*count(L)*count(R), L=left subtree size from heap shape.
// Time: O(N^2) (binomial table + recursion), Space: O(N^2).
function leftSize(n) {
  if (n === 1) return 0;
  const h = Math.floor(Math.log2(n));
  const lower = 1 << (h - 1);
  const last = n - ((1 << h) - 1);
  const leftLast = Math.min(last, lower);
  return ((1 << (h - 1)) - 1) + leftLast;
}

function countHeaps(n, C) {
  if (n <= 1) return 1;
  const L = leftSize(n);
  const R = n - 1 - L;
  return C[n - 1][L] * countHeaps(L, C) * countHeaps(R, C);
}

const N = 3;
const integers = [1, 2, 3];
const C = Array.from({ length: N + 1 }, () => new Array(N + 1).fill(0));
for (let i = 0; i <= N; i++) {
  C[i][0] = 1;
  for (let j = 1; j <= i; j++) C[i][j] = C[i - 1][j - 1] + (j <= i - 1 ? C[i - 1][j] : 0);
}
console.log(countHeaps(N, C));
