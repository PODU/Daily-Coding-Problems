// Day 223: In-order traversal with O(1) extra space (Morris traversal).
// Approach: thread each node to its in-order predecessor temporarily, remove thread after visiting.
// Time O(n), Space O(1) (no stack/recursion).
class Node {
  constructor(val, left = null, right = null) {
    this.val = val;
    this.left = left;
    this.right = right;
  }
}

function morrisInorder(root) {
  const res = [];
  let cur = root;
  while (cur) {
    if (!cur.left) {
      res.push(cur.val);
      cur = cur.right;
    } else {
      let pred = cur.left;
      while (pred.right && pred.right !== cur) pred = pred.right;
      if (!pred.right) {
        pred.right = cur;   // create thread
        cur = cur.left;
      } else {
        pred.right = null;  // remove thread
        res.push(cur.val);
        cur = cur.right;
      }
    }
  }
  return res;
}

const root = new Node(4, new Node(2, new Node(1), new Node(3)), new Node(6, new Node(5), new Node(7)));
console.log(morrisInorder(root)); // [1, 2, 3, 4, 5, 6, 7]
