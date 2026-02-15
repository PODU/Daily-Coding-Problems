// Iterative Fibonacci with two rolling variables. O(n) time, O(1) space.
function fib(n) {
    if (n === 0) return 0;
    let a = 0, b = 1;
    for (let i = 2; i <= n; i++) { let c = a + b; a = b; b = c; }
    return b;
}

console.log(Array.from({length: 11}, (_, i) => fib(i)).join(" "));
console.log(`fib(10) = ${fib(10)}`);
