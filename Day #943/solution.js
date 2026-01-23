// Day 943: Count tilings of a 2xN board with 2x1 dominoes and L-trominoes.
// DP recurrence f(n) = 2*f(n-1) + f(n-3), f(0)=1,f(1)=1,f(2)=2. Time O(n), Space O(1).
function countTilings(n) {
  if (n === 0) return 1;
  if (n === 1) return 1;
  if (n === 2) return 2;
  let a = 1, b = 1, c = 2;
  for (let i = 3; i <= n; i++) {
    const f = 2 * c + a;
    a = b; b = c; c = f;
  }
  return c;
}

console.log(countTilings(4)); // 11
