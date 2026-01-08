// Largest BST subtree via post-order returning (isBST, size, min, max). Time O(n), Space O(h).
public class Solution {
    static class Node {
        int val;
        Node left, right;
        Node(int v){ val = v; }
    }

    static class Info {
        boolean isBST;
        int size, mn, mx;
        Info(boolean b, int s, int mn, int mx){ isBST = b; size = s; this.mn = mn; this.mx = mx; }
    }

    static int best = 0;

    static Info dfs(Node n){
        if(n == null) return new Info(true, 0, Integer.MAX_VALUE, Integer.MIN_VALUE);
        Info l = dfs(n.left), r = dfs(n.right);
        if(l.isBST && r.isBST && n.val > l.mx && n.val < r.mn){
            int sz = l.size + r.size + 1;
            best = Math.max(best, sz);
            return new Info(true, sz, Math.min(n.val, l.mn), Math.max(n.val, r.mx));
        }
        return new Info(false, 0, 0, 0);
    }

    static int largestBST(Node root){
        best = 0;
        dfs(root);
        return best;
    }

    public static void main(String[] args){
        Node root = new Node(10);
        root.left = new Node(5); root.right = new Node(15);
        root.left.left = new Node(1); root.left.right = new Node(8);
        root.right.right = new Node(7);
        System.out.println(largestBST(root));
    }
}
