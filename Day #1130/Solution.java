// Largest BST subtree size via post-order DFS returning {isBST,size,min,max}. O(n) time.
public class Solution {
    static class Node { int val; Node l, r; Node(int v){ val=v; } }
    static class Info { boolean isBST; int size, mn, mx; Info(boolean b,int s,int mn,int mx){ isBST=b; size=s; this.mn=mn; this.mx=mx; } }

    static int best = 0;
    static Info dfs(Node n){
        if(n == null) return new Info(true, 0, Integer.MAX_VALUE, Integer.MIN_VALUE);
        Info L = dfs(n.l), R = dfs(n.r);
        if(L.isBST && R.isBST && n.val > L.mx && n.val < R.mn){
            int sz = L.size + R.size + 1;
            best = Math.max(best, sz);
            return new Info(true, sz, Math.min(n.val, L.mn), Math.max(n.val, R.mx));
        }
        return new Info(false, 0, 0, 0);
    }

    public static void main(String[] args){
        Node root = new Node(10);
        root.l = new Node(5); root.r = new Node(15);
        root.l.l = new Node(1); root.l.r = new Node(8);
        root.r.r = new Node(7);
        dfs(root);
        System.out.println(best);
    }
}
