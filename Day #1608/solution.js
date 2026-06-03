// Count distinct max-heaps from N distinct integers. ways(n)=C(n-1,L)*ways(L)*ways(R),
// L = left-subtree node count of a complete binary tree of n nodes. Time O(n^2), Space O(n^2).
const C = Array.from({ length: 64 }, () => new Array(64).fill(0));
for (let i = 0; i < 64; i++) {
  C[i][0] = 1;
  for (let j = 1; j <= i; j++) C[i][j] = C[i - 1][j - 1] + C[i - 1][j];
}

function leftCount(n) {
  if (n === 1) return 0;
  const h = Math.floor(Math.log2(n));      // height (root at level 0)
  const last = n - ((1 << h) - 1);         // nodes in the bottom level
  const maxLast = 1 << (h - 1);            // max bottom-level nodes for left subtree
  return ((1 << (h - 1)) - 1) + Math.min(last, maxLast);
}

function ways(n) {
  if (n <= 1) return 1;
  const L = leftCount(n), R = n - 1 - L;
  return C[n - 1][L] * ways(L) * ways(R);
}

const arr = [1, 2, 3]; // N distinct integers
console.log(ways(arr.length));
