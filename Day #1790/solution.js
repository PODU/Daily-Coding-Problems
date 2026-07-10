// fib(n): iterative two-variable rolling sum.
// Time O(N), Space O(1).
function fib(n) {
  let a = 0, b = 1;
  for (let i = 0; i < n; i++) {
    [a, b] = [b, a + b];
  }
  return a;
}

console.log(fib(10));
