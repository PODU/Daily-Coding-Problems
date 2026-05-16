// Second largest in BST via parent-walk: find largest; if it has a left subtree,
// answer = max of that subtree, else answer = parent of largest. Time O(h), Space O(1).
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

    static int maxNode(Node n) {
        while (n.right != null) n = n.right;
        return n.val;
    }

    static int secondLargest(Node root) {
        Node cur = root, parent = null;
        while (cur.right != null) {
            parent = cur;
            cur = cur.right;
        }
        if (cur.left != null) return maxNode(cur.left);
        return parent.val;
    }

    public static void main(String[] args) {
        int[] vals = {5, 3, 8, 2, 4, 7, 9};
        Node root = null;
        for (int v : vals) root = insert(root, v);
        System.out.println(secondLargest(root));
    }
}
