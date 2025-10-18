// Day 453: Two nodes in a BST summing to K.
// Inorder -> sorted array, then two-pointer. Time O(n), Space O(n).
class Node {
  constructor(val, left = null, right = null) {
    this.val = val;
    this.left = left;
    this.right = right;
  }
}

function inorder(n, out) {
  if (!n) return;
  inorder(n.left, out);
  out.push(n.val);
  inorder(n.right, out);
}

function twoSum(root, k) {
  const a = [];
  inorder(root, a);
  let i = 0, j = a.length - 1;
  while (i < j) {
    const s = a[i] + a[j];
    if (s === k) return [a[i], a[j]];
    if (s < k) i++; else j--;
  }
  return null;
}

const root = new Node(10, new Node(5), new Node(15, new Node(11), new Node(15)));
const [x, y] = twoSum(root, 20);
console.log(`${x} and ${y}`); // 5 and 15
