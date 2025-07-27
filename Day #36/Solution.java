// Second largest in BST: walk right to largest; second largest is parent of
// largest (if no left subtree) else max of largest's left subtree. O(h) time, O(1) space.
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

    static int secondLargest(Node root) {
        Node parent = null, cur = root;
        while (cur.right != null) { parent = cur; cur = cur.right; }
        if (cur.left != null) {
            cur = cur.left;
            while (cur.right != null) cur = cur.right;
            return cur.val;
        }
        return parent.val;
    }

    public static void main(String[] args) {
        int[] vals = {5, 3, 8, 2, 4, 7, 9};
        Node root = null;
        for (int v : vals) root = insert(root, v);
        System.out.println(secondLargest(root));
    }
}
