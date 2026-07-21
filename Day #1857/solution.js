// Day 1857: Evaluate Reverse Polish Notation.
// Stack: push numbers, pop two on operator and apply. O(n) time, O(n) space.

function evalRPN(tokens) {
  const ops = {
    '+': (a, b) => a + b,
    '-': (a, b) => a - b,
    '*': (a, b) => a * b,
    '/': (a, b) => Math.trunc(a / b), // truncate toward zero
  };
  const st = [];
  for (const t of tokens) {
    if (ops[t]) {
      const b = st.pop();
      const a = st.pop();
      st.push(ops[t](a, b));
    } else {
      st.push(Number(t));
    }
  }
  return st[st.length - 1];
}

const tokens = [15, 7, 1, 1, '+', '-', '/', 3, '*', 2, 1, 1, '+', '+', '-'];
console.log(evalRPN(tokens)); // 5
