// Evaluate Reverse Polish Notation with a stack. Time O(n), Space O(n).

function evalRPN(tokens) {
  const st = [];
  const ops = new Set(["+", "-", "*", "/"]);
  for (const t of tokens) {
    if (ops.has(t)) {
      const b = st.pop();
      const a = st.pop();
      let r;
      if (t === "+") r = a + b;
      else if (t === "-") r = a - b;
      else if (t === "*") r = a * b;
      else r = Math.trunc(a / b);
      st.push(r);
    } else {
      st.push(parseInt(t, 10));
    }
  }
  return st[st.length - 1];
}

const tokens = ["15", "7", "1", "1", "+", "-", "/", "3", "*", "2", "1", "1", "+", "+", "-"];
console.log(evalRPN(tokens));
