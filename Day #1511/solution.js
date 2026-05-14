// Range Sum of BST via DFS with BST pruning (skip left if node<a, right if node>b).
// O(n) worst-case time, O(h) recursion space.

class Node {
    constructor(val) {
        this.val = val;
        this.left = null;
        this.right = null;
    }
}

function rangeSum(root, a, b) {
    if (root === null) return 0;
    if (root.val < a) return rangeSum(root.right, a, b);
    if (root.val > b) return rangeSum(root.left, a, b);
    return root.val + rangeSum(root.left, a, b) + rangeSum(root.right, a, b);
}

const root = new Node(5);
root.left = new Node(3);
root.right = new Node(8);
root.left.left = new Node(2);
root.left.right = new Node(4);
root.right.left = new Node(6);
root.right.right = new Node(10);
console.log(rangeSum(root, 4, 9));
