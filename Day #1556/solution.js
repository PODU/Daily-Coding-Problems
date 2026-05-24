// Merge two binary trees by summing overlapping nodes; recurse and reuse the
// non-null subtree when only one side exists. Time O(n), Space O(h).
"use strict";

class Node {
  constructor(val) {
    this.val = val;
    this.left = null;
    this.right = null;
  }
}

function merge(a, b) {
  if (a === null) return b;
  if (b === null) return a;
  const n = new Node(a.val + b.val);
  n.left = merge(a.left, b.left);
  n.right = merge(a.right, b.right);
  return n;
}

function main() {
  const t1 = new Node(1);
  t1.left = new Node(3);
  t1.right = new Node(2);
  t1.left.left = new Node(5);

  const t2 = new Node(2);
  t2.left = new Node(1);
  t2.right = new Node(3);
  t2.left.right = new Node(4);
  t2.right.right = new Node(7);

  const m = merge(t1, t2);

  const out = [];
  const q = [m];
  while (q.length) {
    const cur = q.shift();
    if (cur !== null) {
      out.push(String(cur.val));
      q.push(cur.left);
      q.push(cur.right);
    } else {
      out.push("null");
    }
  }
  while (out.length && out[out.length - 1] === "null") out.pop();

  console.log(out.join(" "));
}

main();
