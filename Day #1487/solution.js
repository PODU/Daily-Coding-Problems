// Day 1487: In-order traversal in O(1) space via Morris traversal.
// Time: O(n). Space: O(1) extra.

class Node {
  constructor(val, left = null, right = null) {
    this.val = val;
    this.left = left;
    this.right = right;
  }
}

function morrisInorder(root) {
  const out = [];
  let cur = root;
  while (cur) {
    if (cur.left === null) {
      out.push(cur.val);
      cur = cur.right;
    } else {
      let pred = cur.left;
      while (pred.right && pred.right !== cur) pred = pred.right;
      if (pred.right === null) {
        pred.right = cur;       // thread
        cur = cur.left;
      } else {
        pred.right = null;      // restore
        out.push(cur.val);
        cur = cur.right;
      }
    }
  }
  return out;
}

//        4
//       / \
//      2   6
//     / \ / \
//    1  3 5  7
const root = new Node(4,
  new Node(2, new Node(1), new Node(3)),
  new Node(6, new Node(5), new Node(7)));

console.log("In-order:", morrisInorder(root).join(" "));
