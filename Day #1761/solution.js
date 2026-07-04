// Day 1761: Interleave first half of a stack with the reversed second half,
// in-place using ONE auxiliary queue (only push/pop/enqueue/dequeue).
// Dump stack into queue (reverses), rebuild taking alternately back/front.
// Time O(n^2), Space O(n). Stack modeled as array (end = top); queue via index head.
function interleave(input) {
  const stack = [...input]; // end = top
  const q = [];
  let head = 0;
  const size = () => q.length - head;
  while (stack.length) q.push(stack.pop()); // pop top -> enqueue
  let takeBack = true;
  while (size() > 0) {
    let v;
    if (takeBack) {
      const r = size() - 1;
      for (let i = 0; i < r; i++) q.push(q[head++]);
      v = q[head++];
    } else {
      v = q[head++];
    }
    stack.push(v);
    takeBack = !takeBack;
  }
  return stack;
}

const fmt = (v) => `[${v.join(", ")}]`;
console.log(fmt(interleave([1, 2, 3, 4, 5])));
console.log(fmt(interleave([1, 2, 3, 4])));
