// Day 481: Evaluate Reverse Polish Notation using a stack.
// Approach: push operands; on operator pop two and apply. Time O(n), Space O(n).
function evalRPN(tokens) {
  const ops = {
    '+': (a, b) => a + b,
    '-': (a, b) => a - b,
    '*': (a, b) => a * b,
    '/': (a, b) => Math.trunc(a / b),
  };
  const stack = [];
  for (const t of tokens) {
    if (ops[t]) {
      const b = stack.pop();
      const a = stack.pop();
      stack.push(ops[t](a, b));
    } else {
      stack.push(Number(t));
    }
  }
  return stack.pop();
}

const tokens = [15, 7, 1, 1, '+', '-', '/', 3, '*', 2, 1, 1, '+', '+', '-'];
console.log(evalRPN(tokens)); // 5
