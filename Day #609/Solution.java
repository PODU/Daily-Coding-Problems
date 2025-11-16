// Day 609: Inorder successor in a BST using parent pointers.
// Approach: if right child exists, take its leftmost; else climb until coming from a left child. Time O(h).
public class Solution {
    static class Node {
        int val;
        Node left, right, parent;
        Node(int v) { val = v; }
    }

    static Node inorderSuccessor(Node node) {
        if (node == null) return null;
        if (node.right != null) {
            Node c = node.right;
            while (c.left != null) c = c.left;
            return c;
        }
        Node cur = node, p = node.parent;
        while (p != null && cur == p.right) { cur = p; p = p.parent; }
        return p;
    }

    static Node attach(Node parent, Node child) {
        if (child != null) child.parent = parent;
        return child;
    }

    public static void main(String[] args) {
        Node n10 = new Node(10), n5 = new Node(5), n30 = new Node(30);
        Node n22 = new Node(22), n35 = new Node(35);
        n10.left = attach(n10, n5);
        n10.right = attach(n10, n30);
        n30.left = attach(n30, n22);
        n30.right = attach(n30, n35);

        Node s = inorderSuccessor(n22);
        System.out.println(s != null ? s.val : "null"); // 30
    }
}
