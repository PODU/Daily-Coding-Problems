// Post-order DFS: compute each subtree sum, tally counts in a Map, return most frequent.
// Time O(n), Space O(n).
'use strict';

class Node {
    constructor(val, left = null, right = null) {
        this.val = val;
        this.left = left;
        this.right = right;
    }
}

function mostFrequentSubtreeSum(root) {
    const counts = new Map();
    function dfs(node) {
        if (!node) return 0;
        const s = node.val + dfs(node.left) + dfs(node.right);
        counts.set(s, (counts.get(s) || 0) + 1);
        return s;
    }
    dfs(root);
    let best = -Infinity, ans = 0;
    for (const [k, v] of counts) {
        if (v > best || (v === best && k < ans)) {
            best = v;
            ans = k;
        }
    }
    return ans;
}

const root = new Node(5, new Node(2), new Node(-5));
console.log(mostFrequentSubtreeSum(root));
