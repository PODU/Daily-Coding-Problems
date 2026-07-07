// Morris in-order traversal: thread tree via predecessor links for O(1) space.
// Time O(N), Space O(1) (excluding output).
class Node {
    constructor(val) { this.val = val; this.left = null; this.right = null; }
}

function morrisInorder(root) {
    const out = [];
    let cur = root;
    while (cur) {
        if (!cur.left) {
            out.push(cur.val);
            cur = cur.right;
        } else {
            let pre = cur.left;
            while (pre.right && pre.right !== cur) pre = pre.right;
            if (pre.right === null) { pre.right = cur; cur = cur.left; }
            else { pre.right = null; out.push(cur.val); cur = cur.right; }
        }
    }
    console.log(out.join(' '));
}

const root = new Node(4);
root.left = new Node(2);
root.right = new Node(5);
root.left.left = new Node(1);
root.left.right = new Node(3);
morrisInorder(root);
