// Day 278: Generate all structurally distinct BSTs with N nodes (values 1..N).
// Recursive divide on root choice. Count = Catalan(N). Time O(Catalan(N)*N).
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
        for (int r = lo; r <= hi; r++)
            for (Node L : build(lo, r - 1))
                for (Node R : build(r + 1, hi)) {
                    Node n = new Node(r);
                    n.left = L; n.right = R;
                    res.add(n);
                }
        return res;
    }

    static void preorder(Node n, StringBuilder s) {
        if (n == null) { s.append("# "); return; }
        s.append(n.val).append(" ");
        preorder(n.left, s);
        preorder(n.right, s);
    }

    public static void main(String[] args) {
        int N = 3;
        List<Node> trees = build(1, N);
        System.out.println("Count: " + trees.size()); // 5
        for (Node t : trees) {
            StringBuilder s = new StringBuilder();
            preorder(t, s);
            System.out.println(s.toString().trim());
        }
    }
}
