// Day 1444: Convert a binary tree to a full binary tree by removing every node
// with exactly one child (its single child is promoted up).
// Post-order recursion. Time O(n), Space O(h).
import java.util.*;

public class Solution {
    static class Node {
        int val; Node left, right;
        Node(int v) { val = v; }
    }

    static Node toFull(Node root) {
        if (root == null) return null;
        root.left = toFull(root.left);
        root.right = toFull(root.right);
        if (root.left != null && root.right == null) return root.left;
        if (root.left == null && root.right != null) return root.right;
        return root;
    }

    static void preorder(Node r, List<Integer> out) {
        if (r == null) return;
        out.add(r.val);
        preorder(r.left, out);
        preorder(r.right, out);
    }

    public static void main(String[] args) {
        Node n0 = new Node(0);
        n0.left = new Node(1); n0.right = new Node(2);
        n0.left.left = new Node(3);
        n0.left.left.right = new Node(5);
        n0.right.right = new Node(4);
        n0.right.right.left = new Node(6);
        n0.right.right.right = new Node(7);

        Node full = toFull(n0);
        List<Integer> out = new ArrayList<>();
        preorder(full, out);
        StringBuilder sb = new StringBuilder("Preorder of full tree:");
        for (int v : out) sb.append(" ").append(v);
        System.out.println(sb); // 0 5 4 6 7
    }
}
