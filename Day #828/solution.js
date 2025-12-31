// Day 828: Count distinct max heaps from N distinct integers.
// Root is the max; f(n) = C(n-1, L) * f(L) * f(R), L = left-subtree size from
// complete-tree shape. Time O(N^2) (Pascal), Space O(N^2).
function countMaxHeaps(n) {
  const C = Array.from({ length: n + 1 }, () => new Array(n + 1).fill(0));
  for (let i = 0; i <= n; i++) {
    C[i][0] = 1;
    for (let j = 1; j <= i; j++) C[i][j] = C[i - 1][j - 1] + C[i - 1][j];
  }

  function leftCount(m) {
    if (m === 1) return 0;
    const h = 31 - Math.clz32(m);          // height (0-indexed levels)
    const last = m - ((1 << h) - 1);
    const leftCap = 1 << (h - 1);
    return ((1 << (h - 1)) - 1) + Math.min(leftCap, last);
  }

  function f(m) {
    if (m <= 1) return 1;
    const L = leftCount(m);
    const R = m - 1 - L;
    return C[m - 1][L] * f(L) * f(R);
  }

  return f(n);
}

console.log(countMaxHeaps(3));
