// Day 442: Cartesian tree (min-heap ordered, in-order == S) built with a
// monotonic stack in O(n) time, O(n) space.

class Node {
  constructor(val) { this.val = val; this.left = null; this.right = null; }
}

function buildCartesian(s) {
  const stack = [];
  for (const v of s) {
    const node = new Node(v);
    let last = null;
    while (stack.length && stack[stack.length - 1].val > v) last = stack.pop();
    node.left = last;
    if (stack.length) stack[stack.length - 1].right = node;
    stack.push(node);
  }
  return stack.length ? stack[0] : null;
}

function show(n, prefix = "", tag = "") {
  if (!n) return;
  console.log(prefix + tag + n.val);
  show(n.left, prefix + "  ", "L-- ");
  show(n.right, prefix + "  ", "R-- ");
}

const root = buildCartesian([3, 2, 6, 1, 9]);
show(root);
// 1
//   L-- 2
//     L-- 3
//     R-- 6
//   R-- 9
