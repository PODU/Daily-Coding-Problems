// Postorder DFS: each node returns val+max(0,L,R) upward; global best = val+max(0,L)+max(0,R); O(n) time O(h) space

class TreeNode {
    constructor(val, left = null, right = null) {
        this.val = val;
        this.left = left;
        this.right = right;
    }
}

function maxPathSum(root) {
    let best = -Infinity;
    function dfs(node) {
        if (!node) return 0;
        const l = Math.max(0, dfs(node.left));
        const r = Math.max(0, dfs(node.right));
        best = Math.max(best, node.val + l + r);
        return node.val + Math.max(l, r);
    }
    dfs(root);
    return best;
}

//       -10
//       /  \
//      9    20
//          /  \
//         15   7
const root1 = new TreeNode(-10);
root1.left = new TreeNode(9);
root1.right = new TreeNode(20);
root1.right.left = new TreeNode(15);
root1.right.right = new TreeNode(7);
console.log(`Max path sum: ${maxPathSum(root1)}`);

//    1
//   / \
//  2   3
const root2 = new TreeNode(1);
root2.left = new TreeNode(2);
root2.right = new TreeNode(3);
console.log(`Max path sum: ${maxPathSum(root2)}`);
