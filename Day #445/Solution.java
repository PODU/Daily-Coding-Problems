// Day 445: Prune a 0/1 binary tree, removing all-zero subtrees.
// Postorder recursion, O(n) time, O(h) space.
public class Solution {
    static class Node {
        int val; Node left, right;
        Node(int v) { val = v; }
    }

    static Node prune(Node n) {
        if (n == null) return null;
        n.left = prune(n.left);
        n.right = prune(n.right);
        if (n.val == 0 && n.left == null && n.right == null) return null;
        return n;
    }

    static void show(Node n, String prefix, String tag) {
        if (n == null) return;
        System.out.println(prefix + tag + n.val);
        show(n.left, prefix + "  ", "L-- ");
        show(n.right, prefix + "  ", "R-- ");
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
        show(root, "", "");
        // 0 / L-- 1 / R-- 0 (L-- 1)
    }
}
