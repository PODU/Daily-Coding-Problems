// Day 527: Count distinct max-heaps from N distinct integers.
// Recurrence f(n) = C(n-1, L) * f(L) * f(R), L = left-subtree size of a complete
// binary tree with n nodes. BigInt for exactness. Time O(n^2), space O(n).

function leftSubtreeSize(n) {
  // number of nodes in the left subtree of a complete binary tree with n nodes
  if (n <= 1) return 0;
  let h = 0;
  while ((1 << (h + 1)) - 1 <= n) h++; // h = height (root at height 0)
  const lastLevelCap = 1 << h;
  const nodesAbove = (1 << h) - 1;
  const lastLevelNodes = n - nodesAbove;
  const leftBase = (1 << (h - 1)) - 1;
  const leftLast = Math.min(lastLevelNodes, lastLevelCap / 2);
  return leftBase + leftLast;
}

function binom(n, k) {
  let r = 1n;
  for (let i = 0; i < k; i++) r = (r * BigInt(n - i)) / BigInt(i + 1);
  return r;
}

function countHeaps(n) {
  if (n <= 1) return 1n;
  const L = leftSubtreeSize(n);
  const R = n - 1 - L;
  return binom(n - 1, L) * countHeaps(L) * countHeaps(R);
}

const N = 3;
const integers = [1, 2, 3];
console.log(countHeaps(N).toString()); // expected: 2
