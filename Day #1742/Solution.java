// Approach: recursive generation of all BSTs; root choice + Cartesian product of left/right.
// Time/Space O(Catalan(N)*N).
import java.util.ArrayList;
import java.util.List;

public class Solution {
    static class Node {
        int val;
        Node left, right;
        Node(int v) { val = v; }
    }

    static List<Node> build(int lo, int hi) {
        List<Node> res = new ArrayList<>();
        if (lo > hi) { res.add(null); return res; }
        for (int r = lo; r <= hi; r++) {
            List<Node> lefts = build(lo, r - 1);
            List<Node> rights = build(r + 1, hi);
            for (Node l : lefts)
                for (Node ri : rights) {
                    Node root = new Node(r);
                    root.left = l;
                    root.right = ri;
                    res.add(root);
                }
        }
        return res;
    }

    static void preorder(Node n, StringBuilder sb) {
        if (n == null) return;
        if (sb.length() > 0) sb.append(',');
        sb.append(n.val);
        preorder(n.left, sb);
        preorder(n.right, sb);
    }

    public static void main(String[] args) {
        int N = 3;
        List<Node> trees = build(1, N);
        System.out.println(trees.size());
        for (Node t : trees) {
            StringBuilder sb = new StringBuilder();
            preorder(t, sb);
            System.out.println(sb.toString());
        }
    }
}
