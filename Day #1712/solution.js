// In-order traversal -> sorted array, then two-pointer for pair summing to K. Time O(n), Space O(n).
class Node {
  constructor(val) {
    this.val = val;
    this.left = null;
    this.right = null;
  }
}

function inorder(root, acc) {
  if (!root) return;
  inorder(root.left, acc);
  acc.push(root.val);
  inorder(root.right, acc);
}

function findPair(root, k) {
  const a = [];
  inorder(root, a);
  let i = 0, j = a.length - 1;
  while (i < j) {
    const s = a[i] + a[j];
    if (s === k) return [a[i], a[j]];
    else if (s < k) i++;
    else j--;
  }
  return null;
}

const root = new Node(10);
root.left = new Node(5);
root.right = new Node(15);
root.right.left = new Node(11);
root.right.right = new Node(15);
const pair = findPair(root, 20);
console.log(`${pair[0]} and ${pair[1]}`);
