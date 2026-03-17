// Iterative rolling Fibonacci: two variables, O(n) time, O(1) space.
// fib(0)=0, fib(1)=1.
function fib(n) {
  if (n < 2) return n;
  let a = 0, b = 1;
  for (let i = 2; i <= n; i++) {
    const c = a + b;
    a = b;
    b = c;
  }
  return b;
}

console.log(fib(10));
