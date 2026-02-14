// Post-order prune: remove subtrees consisting entirely of zeros. Time: O(n), Space: O(n) stack.
class TreeNode {
    constructor(val, left=null, right=null) { this.val=val; this.left=left; this.right=right; }
}

function prune(n) {
    if (!n) return null;
    n.left  = prune(n.left);
    n.right = prune(n.right);
    if (n.val === 0 && !n.left && !n.right) return null;
    return n;
}

function preorder(n, out) {
    if (!n) { out.push('X'); return; }
    out.push(String(n.val));
    preorder(n.left,  out);
    preorder(n.right, out);
}

const root = new TreeNode(0,
    new TreeNode(1),
    new TreeNode(0,
        new TreeNode(1, new TreeNode(0), new TreeNode(0)),
        new TreeNode(0)
    )
);

const pruned = prune(root);
const out = [];
preorder(pruned, out);
console.log('Pruned preorder (X=null): ' + out.join(' '));
