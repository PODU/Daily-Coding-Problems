// 2xN tiling with dominoes + L-trominoes (LeetCode 790). DP recurrence
// f(n)=2*f(n-1)+f(n-3). O(N) time, O(1) space. Mod 1e9+7 for large N.
const MOD = 1000000007n;

function numTilings(n) {
  if (n === 0) return 1n;
  if (n === 1) return 1n;
  if (n === 2) return 2n;
  let a = 1n, b = 1n, c = 2n; // f(0),f(1),f(2)
  for (let i = 3; i <= n; i++) {
    const f = (2n * c + a) % MOD;
    a = b; b = c; c = f;
  }
  return c;
}

console.log("N=4 -> " + numTilings(4)); // 11
const table = [1, 2, 3, 4, 5].map((n) => numTilings(n).toString()).join(" ");
console.log("Table N=1..5: " + table); // 1 2 5 11 24
