// Day 484: Second largest node in a BST.
// O(h): walk right to largest; second largest is its parent, or max of largest's left subtree.
// Time O(h), Space O(1).
public class Solution {
    static class Node {
        int val;
        Node left, right;
        Node(int v) { val = v; }
    }

    static Node insert(Node root, int v) {
        if (root == null) return new Node(v);
        if (v < root.val) root.left = insert(root.left, v);
        else root.right = insert(root.right, v);
        return root;
    }

    static Node secondLargest(Node root) {
        if (root == null || (root.left == null && root.right == null)) return null;
        Node cur = root, parent = null;
        while (cur.right != null) { parent = cur; cur = cur.right; }
        if (cur.left != null) {
            cur = cur.left;
            while (cur.right != null) cur = cur.right;
            return cur;
        }
        return parent;
    }

    public static void main(String[] args) {
        Node root = null;
        for (int v : new int[]{5, 3, 8, 2, 4, 7, 10}) root = insert(root, v);
        System.out.println(secondLargest(root).val); // 8
    }
}
