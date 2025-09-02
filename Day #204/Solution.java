// Day 204: Count nodes of a complete binary tree faster than O(n).
// Compare left/right spine heights: if equal subtree is perfect (2^h - 1), else recurse.
// Time: O(log^2 n), Space: O(log n).
public class Solution {
    static class Node { Node left, right; }

    static int leftHeight(Node n) { int h = 0; while (n != null) { h++; n = n.left; } return h; }
    static int rightHeight(Node n) { int h = 0; while (n != null) { h++; n = n.right; } return h; }

    static int countNodes(Node root) {
        if (root == null) return 0;
        int lh = leftHeight(root), rh = rightHeight(root);
        if (lh == rh) return (1 << lh) - 1; // perfect subtree
        return 1 + countNodes(root.left) + countNodes(root.right);
    }

    public static void main(String[] args) {
        Node[] n = new Node[7];
        for (int i = 1; i < 7; i++) n[i] = new Node();
        n[1].left = n[2]; n[1].right = n[3];
        n[2].left = n[4]; n[2].right = n[5];
        n[3].left = n[6];
        System.out.println(countNodes(n[1])); // 6
    }
}
