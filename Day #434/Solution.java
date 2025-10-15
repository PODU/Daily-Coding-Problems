// Day 434: BST floor (largest <= n) and ceiling (smallest >= n).
// Single O(h) walk: node.val < n -> floor candidate (go right); node.val > n -> ceiling
// candidate (go left); equal -> both are n. O(h) time, O(1) space.
public class Solution {
    static class Node { int val; Node left, right; Node(int v) { val = v; } }

    static Node insert(Node root, int v) {
        if (root == null) return new Node(v);
        if (v < root.val) root.left = insert(root.left, v);
        else root.right = insert(root.right, v);
        return root;
    }

    static Integer[] floorCeil(Node root, int n) {
        Integer f = null, c = null;
        Node cur = root;
        while (cur != null) {
            if (cur.val == n) return new Integer[]{n, n};
            else if (cur.val < n) { f = cur.val; cur = cur.right; }
            else { c = cur.val; cur = cur.left; }
        }
        return new Integer[]{f, c};
    }

    public static void main(String[] args) {
        Node root = null;
        for (int v : new int[]{8, 4, 12, 2, 6, 10, 14}) root = insert(root, v);
        for (int n : new int[]{5, 8, 15, 1}) {
            Integer[] fc = floorCeil(root, n);
            System.out.println("n=" + n + ": floor=" + (fc[0] == null ? "None" : fc[0])
                    + ", ceiling=" + (fc[1] == null ? "None" : fc[1]));
        }
    }
}
