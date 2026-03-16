// Day 1211: Interleave first half of a stack with the reversed second half, using one queue.
// Load stack bottom->top into the queue, then alternately push front/back (back via rotation). Time O(n^2), Space O(n).
function interleave(st) {
  const q = []; // queue: enqueue=push, dequeue=shift
  let n = 0;
  while (st.length) { q.push(st.pop()); n++; }      // front = old top
  for (let i = 0; i < n; i++) st.push(q.shift());    // top = old bottom
  for (let i = 0; i < n; i++) q.push(st.pop());      // front = bottom
  let remaining = n, takeFront = true;
  while (remaining > 0) {
    if (takeFront) st.push(q.shift());
    else {
      for (let i = 0; i < remaining - 1; i++) q.push(q.shift());
      st.push(q.shift());
    }
    remaining--; takeFront = !takeFront;
  }
}

const st = [];
for (const x of [1, 2, 3, 4, 5]) st.push(x); // bottom -> top
interleave(st);
console.log(st); // [ 1, 5, 2, 4, 3 ]
