// Largest BST subtree size via post-order DFS returning {isBST,size,min,max}. O(n) time.
'use strict';

class Node { constructor(val){ this.val = val; this.l = null; this.r = null; } }

let best = 0;
function dfs(n){
    if(n === null) return { isBST: true, size: 0, mn: Infinity, mx: -Infinity };
    const L = dfs(n.l), R = dfs(n.r);
    if(L.isBST && R.isBST && n.val > L.mx && n.val < R.mn){
        const sz = L.size + R.size + 1;
        best = Math.max(best, sz);
        return { isBST: true, size: sz, mn: Math.min(n.val, L.mn), mx: Math.max(n.val, R.mx) };
    }
    return { isBST: false, size: 0, mn: 0, mx: 0 };
}

function main(){
    const root = new Node(10);
    root.l = new Node(5); root.r = new Node(15);
    root.l.l = new Node(1); root.l.r = new Node(8);
    root.r.r = new Node(7);
    dfs(root);
    console.log(best);
}

main();
