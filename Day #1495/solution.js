// Day 1495: Build a min-heap-ordered Cartesian tree whose in-order traversal is S.
// Approach: monotonic stack; pop nodes > current as its left subtree. Time O(n), Space O(n).
class Node {
  constructor(val) { this.val = val; this.left = null; this.right = null; }
}

function buildCartesian(s) {
  const stack = [];
  for (const x of s) {
    const cur = new Node(x);
    let last = null;
    while (stack.length && stack[stack.length - 1].val > x) last = stack.pop();
    cur.left = last;
    if (stack.length) stack[stack.length - 1].right = cur;
    stack.push(cur);
  }
  return stack.length ? stack[0] : null;
}

function inorder(n, out) {
  if (!n) return;
  inorder(n.left, out);
  out.push(n.val);
  inorder(n.right, out);
}

function pretty(n, depth = 0) {
  if (!n) return;
  pretty(n.right, depth + 1);
  console.log(" ".repeat(depth * 4) + n.val);
  pretty(n.left, depth + 1);
}

function listing(n) {
  if (!n) return;
  if (n.left || n.right) {
    const kids = [];
    if (n.left) kids.push(n.left.val);
    if (n.right) kids.push(n.right.val);
    console.log(`  ${n.val} -> ${kids.join(" ")}`);
  }
  listing(n.left);
  listing(n.right);
}

const s = [3, 2, 6, 1, 9];
const root = buildCartesian(s);

const io = [];
inorder(root, io);
console.log("In-order traversal: " + io.join(" "));

console.log("Root: " + root.val);
console.log("Parent -> children:");
listing(root);

console.log("Tree (rotated 90 deg, root at left):");
pretty(root);
