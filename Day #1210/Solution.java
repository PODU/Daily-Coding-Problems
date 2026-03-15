// Day 1210: Floor and ceiling of a target in a BST.
// Single root-to-leaf descent updating best candidates. Time O(h), Space O(1).
public class Solution {
    static class Node { int val; Node l, r; Node(int v) { val = v; } }

    static Node insert(Node root, int v) {
        if (root == null) return new Node(v);
        if (v < root.val) root.l = insert(root.l, v);
        else root.r = insert(root.r, v);
        return root;
    }

    static Integer[] floorCeil(Node root, int x) {
        Integer fl = null, ce = null;
        while (root != null) {
            if (root.val == x) return new Integer[]{x, x};
            if (root.val < x) { fl = root.val; root = root.r; }
            else { ce = root.val; root = root.l; }
        }
        return new Integer[]{fl, ce};
    }

    public static void main(String[] args) {
        Node root = null;
        for (int v : new int[]{8, 4, 12, 2, 6, 10, 14}) root = insert(root, v);
        Integer[] fc = floorCeil(root, 7);
        System.out.println("floor=" + (fc[0] == null ? "None" : fc[0])
                + " ceiling=" + (fc[1] == null ? "None" : fc[1])); // floor=6 ceiling=8
    }
}
