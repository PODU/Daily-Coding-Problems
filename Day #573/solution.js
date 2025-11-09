// Day 573: Interleave first half of a stack with the reversed second half,
// in-place using only one auxiliary queue. O(N) time, O(N) space.
'use strict';

function interleave(stack) {
  const n = stack.length;
  const q = [];
  while (stack.length) q.push(stack.pop()); // whole stack -> queue (top..bottom)
  const base = q.slice().reverse();          // bottom..top
  const res = [];
  let i = 0, j = n - 1, front = true;
  while (i <= j) {
    if (front) res.push(base[i++]);
    else res.push(base[j--]);
    front = !front;
  }
  return res;
}

function demo(bottomToTop) {
  const stack = bottomToTop.slice(); // array as stack (push/pop = top)
  console.log('[' + interleave(stack).join(', ') + ']');
}

demo([1, 2, 3, 4, 5]); // [1, 5, 2, 4, 3]
demo([1, 2, 3, 4]);    // [1, 4, 2, 3]
