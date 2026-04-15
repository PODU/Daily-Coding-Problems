// Count complete-tree nodes: if left height == right height subtree is perfect (2^h-1),
// else recurse. Time O(log^2 n), Space O(log n) recursion.
public class Solution {
    static class Node {
        int val;
        Node left, right;
        Node(int v) { val = v; }
    }

    static int leftHeight(Node n) { int h = 0; while (n != null) { h++; n = n.left; } return h; }
    static int rightHeight(Node n) { int h = 0; while (n != null) { h++; n = n.right; } return h; }

    static int countNodes(Node root) {
        if (root == null) return 0;
        int lh = leftHeight(root), rh = rightHeight(root);
        if (lh == rh) return (1 << lh) - 1;
        return 1 + countNodes(root.left) + countNodes(root.right);
    }

    public static void main(String[] args) {
        Node root = new Node(1);
        root.left = new Node(2);
        root.right = new Node(3);
        root.left.left = new Node(4);
        root.left.right = new Node(5);
        root.right.left = new Node(6);
        System.out.println(countNodes(root));
    }
}
