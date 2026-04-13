// Zigzag (boustrophedon) level order of a binary tree. BFS per level, reverse alternate levels. O(N) time, O(N) space.

class Node {
    constructor(val) {
        this.val = val;
        this.left = null;
        this.right = null;
    }
}

function zigzag(root) {
    const res = [];
    if (!root) return res;
    let queue = [root];
    let leftToRight = true;
    while (queue.length > 0) {
        const sz = queue.length;
        const level = new Array(sz);
        const next = [];
        for (let i = 0; i < sz; i++) {
            const cur = queue[i];
            const idx = leftToRight ? i : sz - 1 - i;
            level[idx] = cur.val;
            if (cur.left) next.push(cur.left);
            if (cur.right) next.push(cur.right);
        }
        for (const v of level) res.push(v);
        queue = next;
        leftToRight = !leftToRight;
    }
    return res;
}

const root = new Node(1);
root.left = new Node(2);
root.right = new Node(3);
root.left.left = new Node(4);
root.left.right = new Node(5);
root.right.left = new Node(6);
root.right.right = new Node(7);

console.log("[" + zigzag(root).join(", ") + "]");
