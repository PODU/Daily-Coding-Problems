// Day 307: BST floor (largest <= x) and ceiling (smallest >= x). O(h) per query.
public class Solution {
    static class Node { int v; Node l, r; Node(int x) { v = x; } }
    static Node insert(Node root, int v) {
        if (root == null) return new Node(v);
        if (v < root.v) root.l = insert(root.l, v); else root.r = insert(root.r, v);
        return root;
    }
    static Integer floor, ceil;
    static void floorCeil(Node root, int x) {
        floor = null; ceil = null;
        while (root != null) {
            if (root.v == x) { floor = ceil = root.v; return; }
            if (root.v < x) { floor = root.v; root = root.r; }
            else { ceil = root.v; root = root.l; }
        }
    }
    public static void main(String[] a) {
        int[] vals = {8, 4, 12, 2, 6, 10, 14}; Node root = null;
        for (int v : vals) root = insert(root, v);
        floorCeil(root, 5);
        System.out.println("Floor: " + (floor == null ? "None" : floor)
                + ", Ceiling: " + (ceil == null ? "None" : ceil)); // Floor: 4, Ceiling: 6
    }
}
