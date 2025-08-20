// Prune subtrees that contain only 0s via post-order recursion. O(n) time, O(h) stack.

public class Solution {
    static class Node {
        int val; Node left, right;
        Node(int v) { val = v; }
    }

    static Node prune(Node root) {
        if (root == null) return null;
        root.left = prune(root.left);
        root.right = prune(root.right);
        if (root.val == 0 && root.left == null && root.right == null) return null;
        return root;
    }

    static void preorder(Node r, StringBuilder sb) {
        if (r == null) return;
        sb.append(r.val).append(' ');
        preorder(r.left, sb);
        preorder(r.right, sb);
    }

    public static void main(String[] args) {
        Node root = new Node(0);
        root.left = new Node(1);
        root.right = new Node(0);
        root.right.left = new Node(1);
        root.right.right = new Node(0);
        root.right.left.left = new Node(0);
        root.right.left.right = new Node(0);
        root = prune(root);
        StringBuilder sb = new StringBuilder();
        preorder(root, sb);
        System.out.println("preorder: " + sb.toString().trim()); // 0 1 0 1
    }
}
