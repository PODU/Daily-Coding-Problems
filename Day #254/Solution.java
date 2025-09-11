// Prune to full binary tree: post-order DFS; a node with exactly one child is
// replaced by that child. Returns new root. Time O(n), Space O(h) recursion.
import java.util.ArrayList;
import java.util.List;

public class Solution {
    static class Node {
        int val;
        Node left, right;
        Node(int v) { val = v; }
    }

    static Node prune(Node root) {
        if (root == null) return null;
        root.left = prune(root.left);
        root.right = prune(root.right);
        if (root.left != null && root.right == null) return root.left;
        if (root.left == null && root.right != null) return root.right;
        return root;
    }

    static void preorder(Node n, List<Integer> out) {
        if (n == null) return;
        out.add(n.val);
        preorder(n.left, out);
        preorder(n.right, out);
    }

    public static void main(String[] args) {
        Node root = new Node(0);
        root.left = new Node(1);
        root.right = new Node(2);
        root.left.left = new Node(3);
        root.left.left.right = new Node(5);
        root.right.right = new Node(4);
        root.right.right.left = new Node(6);
        root.right.right.right = new Node(7);

        root = prune(root);
        List<Integer> pre = new ArrayList<>();
        preorder(root, pre);
        StringBuilder sb = new StringBuilder("Preorder after pruning:");
        for (int v : pre) sb.append(" ").append(v);
        System.out.println(sb.toString());
    }
}
