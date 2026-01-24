// Day 951: interleave first half of a stack with the reversed second half,
// in place using only one auxiliary queue (push/pop, enqueue/dequeue).
// Time O(n^2) due to rotations, Space O(1) extra besides the queue.

function interleave(input) {
  const st = [...input]; // top = end
  const q = []; // queue: push to back, shift from front
  while (st.length) q.push(st.pop());
  while (q.length) st.push(q.shift());
  while (st.length) q.push(st.pop()); // q = [a0..a_{n-1}]
  while (q.length) {
    st.push(q.shift());
    const m = q.length;
    if (m === 0) break;
    for (let i = 0; i < m - 1; i++) q.push(q.shift());
    st.push(q.shift());
  }
  return st; // bottom..top
}

console.log(JSON.stringify(interleave([1, 2, 3, 4, 5]))); // [1,5,2,4,3]
console.log(JSON.stringify(interleave([1, 2, 3, 4])));    // [1,4,2,3]
