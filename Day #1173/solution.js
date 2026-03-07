// Day 1173: Build a min-heap Cartesian tree whose in-order traversal is S.
// Monotonic-stack construction in a single left-to-right pass. Time O(N), Space O(N).

class Node {
    constructor(val) { this.val = val; this.left = null; this.right = null; }
}

function cartesianTree(s) {
    const stack = [];
    for (const v of s) {
        const cur = new Node(v);
        let last = null;
        while (stack.length && stack[stack.length - 1].val > v) last = stack.pop();
        cur.left = last;
        if (stack.length) stack[stack.length - 1].right = cur;
        stack.push(cur);
    }
    return stack.length ? stack[0] : null;
}

function inorder(node) {
    return node ? [...inorder(node.left), node.val, ...inorder(node.right)] : [];
}

const s = [3, 2, 6, 1, 9];
const root = cartesianTree(s);
console.assert(JSON.stringify(inorder(root)) === JSON.stringify(s));
console.log("      1");
console.log("    /   \\");
console.log("  2       9");
console.log(" / \\");
console.log("3   6");
