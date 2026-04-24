// Day 1418: inorder successor of a BST node using parent pointers.
// Approach: if right subtree exists, leftmost of it; else climb until node is a left child. O(h).
public class Solution {
    static class Node {
        int val;
        Node left, right, parent;
        Node(int v) { val = v; }
    }

    static Node successor(Node node) {
        if (node.right != null) {
            Node cur = node.right;
            while (cur.left != null) cur = cur.left;
            return cur;
        }
        Node cur = node;
        while (cur.parent != null && cur == cur.parent.right) cur = cur.parent;
        return cur.parent;
    }

    static Node attach(Node parent, Node child) {
        if (child != null) child.parent = parent;
        return child;
    }

    public static void main(String[] args) {
        Node root = new Node(10);
        root.left = attach(root, new Node(5));
        root.right = attach(root, new Node(30));
        Node n22 = new Node(22);
        root.right.left = attach(root.right, n22);
        root.right.right = attach(root.right, new Node(35));

        Node s = successor(n22);
        System.out.println(s != null ? s.val : "null"); // 30
    }
}
