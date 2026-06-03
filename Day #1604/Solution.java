// BST floor (largest <= x) & ceiling (smallest >= x). Iterative O(h) time, O(1) space.
// Floor: go right recording when val<=x else left. Ceiling: symmetric.
public class Solution {
    static class Node { int val; Node l, r; Node(int v){ val=v; } }

    static Node insert(Node root, int v) {
        if (root == null) return new Node(v);
        if (v < root.val) root.l = insert(root.l, v);
        else root.r = insert(root.r, v);
        return root;
    }

    static Integer floorBST(Node root, int x) {
        Integer res = null;
        while (root != null) {
            if (root.val == x) return x;
            if (root.val < x) { res = root.val; root = root.r; }
            else root = root.l;
        }
        return res;
    }

    static Integer ceilBST(Node root, int x) {
        Integer res = null;
        while (root != null) {
            if (root.val == x) return x;
            if (root.val > x) { res = root.val; root = root.l; }
            else root = root.r;
        }
        return res;
    }

    static String show(Integer v){ return v == null ? "None" : v.toString(); }

    static void query(Node root, int x){
        System.out.println("x=" + x + " -> floor " + show(floorBST(root,x))
            + ", ceiling " + show(ceilBST(root,x)));
    }

    public static void main(String[] args) {
        Node root = null;
        for (int v : new int[]{8,4,12,2,6,10,14}) root = insert(root, v);
        query(root,5);   // floor 4, ceiling 6
        query(root,8);   // floor 8, ceiling 8
        query(root,15);  // floor 14, ceiling None
        query(root,1);   // floor None, ceiling 2
    }
}
