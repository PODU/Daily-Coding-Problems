// Fibonacci in O(1) space: iterate keeping only the last two values.
// Time: O(n), Space: O(1).
function fib(n) {
  if (n < 2) return n;
  let a = 0n, b = 1n;
  for (let i = 2; i <= n; i++) {
    [a, b] = [b, a + b];
  }
  return b;
}

console.log(fib(10).toString()); // 55
