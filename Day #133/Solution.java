// Day 133: Inorder successor in a BST using parent pointers.
// If right subtree exists, leftmost of it; else climb until node is a left child. O(h) time.
public class Solution {
    static class Node {
        int val;
        Node left, right, parent;
        Node(int v) { val = v; }
    }

    static Node successor(Node node) {
        if (node == null) return null;
        if (node.right != null) {
            Node c = node.right;
            while (c.left != null) c = c.left;
            return c;
        }
        Node p = node.parent;
        while (p != null && node == p.right) {
            node = p;
            p = p.parent;
        }
        return p;
    }

    static Node attach(Node parent, Node child) {
        if (child != null) child.parent = parent;
        return child;
    }

    public static void main(String[] args) {
        Node root = new Node(10);
        root.left = attach(root, new Node(5));
        root.right = attach(root, new Node(30));
        Node n22 = new Node(22), n35 = new Node(35);
        root.right.left = attach(root.right, n22);
        root.right.right = attach(root.right, n35);

        Node s = successor(n22);
        System.out.println(s != null ? s.val : "null"); // 30
    }
}
