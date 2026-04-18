// Second largest in BST via reverse in-order (right,node,left), stop at 2nd visited node. O(h) space, O(n) worst time.
class Node {
    constructor(val) { this.val = val; this.left = null; this.right = null; }
}

function secondLargest(root) {
    const st = [];
    let cur = root;
    let count = 0;
    while (cur || st.length) {
        while (cur) { st.push(cur); cur = cur.right; }
        cur = st.pop();
        if (++count === 2) return cur.val;
        cur = cur.left;
    }
    return -1;
}

const root = new Node(5);
root.left = new Node(3);
root.left.left = new Node(2);
root.left.right = new Node(4);
root.right = new Node(8);
root.right.left = new Node(7);
root.right.right = new Node(9);
console.log(secondLargest(root));
