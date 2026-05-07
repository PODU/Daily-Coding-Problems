// Day 1484: Construct all structurally unique BSTs with N nodes (values 1..N).
// For each root i, combine all left BSTs of (lo..i-1) with all right BSTs of
// (i+1..hi). Count is Catalan(N). Time/Space O(Catalan(N) * N).
import java.util.*;

public class Solution {
    static class Node {
        int val;
        Node left, right;
        Node(int v, Node l, Node r) { val = v; left = l; right = r; }
    }

    static List<Node> build(int lo, int hi) {
        List<Node> trees = new ArrayList<>();
        if (lo > hi) { trees.add(null); return trees; }
        for (int i = lo; i <= hi; ++i)
            for (Node l : build(lo, i - 1))
                for (Node r : build(i + 1, hi))
                    trees.add(new Node(i, l, r));
        return trees;
    }

    static void preorder(Node n, List<Integer> out) {
        if (n == null) return;
        out.add(n.val);
        preorder(n.left, out);
        preorder(n.right, out);
    }

    public static void main(String[] args) {
        List<Node> trees = build(1, 3);
        System.out.println(trees.size());  // 5
        for (Node t : trees) {
            List<Integer> out = new ArrayList<>();
            preorder(t, out);
            System.out.println(out);
        }
    }
}
