// Day 1625: Inorder successor in BST using parent pointers.
// If right subtree exists, leftmost of it; else climb until node is left child. O(h).
public class Solution {
    static class Node {
        int val;
        Node left, right, parent;
        Node(int v) { val = v; }
    }

    static Node inorderSuccessor(Node node) {
        if (node == null) return null;
        if (node.right != null) {
            Node cur = node.right;
            while (cur.left != null) cur = cur.left;
            return cur;
        }
        Node cur = node;
        while (cur.parent != null && cur.parent.right == cur) cur = cur.parent;
        return cur.parent;
    }

    static Node insert(Node root, int v) {
        if (root == null) return new Node(v);
        Node cur = root;
        while (true) {
            if (v < cur.val) {
                if (cur.left == null) { cur.left = new Node(v); cur.left.parent = cur; return root; }
                cur = cur.left;
            } else {
                if (cur.right == null) { cur.right = new Node(v); cur.right.parent = cur; return root; }
                cur = cur.right;
            }
        }
    }

    static Node find(Node root, int v) {
        while (root != null && root.val != v) root = v < root.val ? root.left : root.right;
        return root;
    }

    public static void main(String[] args) {
        Node root = null;
        for (int v : new int[]{10, 5, 30, 22, 35}) root = insert(root, v);
        Node s = inorderSuccessor(find(root, 22));
        System.out.println("The inorder successor of 22 is " + (s != null ? s.val : "None") + ".");
    }
}
