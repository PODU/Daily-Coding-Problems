// LCA with parent pointers: equalize depths, then walk both up together.
// Time O(h), Space O(1).
class Node {
  constructor(val) {
    this.val = val;
    this.parent = null;
    this.left = null;
    this.right = null;
  }
}

function depth(n) {
  let d = 0;
  while (n) {
    d++;
    n = n.parent;
  }
  return d;
}

function lca(a, b) {
  let da = depth(a),
    db = depth(b);
  while (da > db) {
    a = a.parent;
    da--;
  }
  while (db > da) {
    b = b.parent;
    db--;
  }
  while (a !== b) {
    a = a.parent;
    b = b.parent;
  }
  return a;
}

function child(parent, c) {
  if (c) c.parent = parent;
  return c;
}

function main() {
  const n3 = new Node(3), n5 = new Node(5), n1 = new Node(1);
  const n6 = new Node(6), n2 = new Node(2), n0 = new Node(0), n8 = new Node(8);
  const n7 = new Node(7), n4 = new Node(4);

  n3.left = child(n3, n5);
  n3.right = child(n3, n1);
  n5.left = child(n5, n6);
  n5.right = child(n5, n2);
  n1.left = child(n1, n0);
  n1.right = child(n1, n8);
  n2.left = child(n2, n7);
  n2.right = child(n2, n4);

  console.log(lca(n6, n4).val);
  console.log(lca(n6, n8).val);
}

main();
