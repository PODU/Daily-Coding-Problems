// Balanced brackets via stack: push openers, match closers to top. O(n) time, O(n) space.
function isBalanced(s) {
  const st = [];
  const pair = { ')': '(', ']': '[', '}': '{' };
  for (const c of s) {
    if (c === '(' || c === '[' || c === '{') st.push(c);
    else if (c in pair) {
      if (st.pop() !== pair[c]) return false;
    }
  }
  return st.length === 0;
}

console.log(isBalanced("([])[]({})"));
console.log(isBalanced("([)]"));
