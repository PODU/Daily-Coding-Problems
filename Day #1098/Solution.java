// Day 1098: Floor and ceiling of x in a BST.
// Walk down the tree updating candidates using BST ordering.
// Time: O(h). Space: O(1).
public class Solution {
    static class Node { int val; Node l, r; Node(int v){ val = v; } }

    static Node insert(Node root, int v){
        if (root == null) return new Node(v);
        if (v < root.val) root.l = insert(root.l, v);
        else root.r = insert(root.r, v);
        return root;
    }

    static Integer[] floorCeil(Node root, int x){
        Integer fl = null, ce = null;
        Node cur = root;
        while (cur != null){
            if (cur.val == x) return new Integer[]{x, x};
            if (cur.val < x){ fl = cur.val; cur = cur.r; }
            else { ce = cur.val; cur = cur.l; }
        }
        return new Integer[]{fl, ce};
    }

    public static void main(String[] args){
        Node root = null;
        for (int v : new int[]{8,4,12,2,6,10,14}) root = insert(root, v);
        Integer[] r = floorCeil(root, 5);
        System.out.println("floor=" + (r[0]==null?"None":r[0]) + " ceil=" + (r[1]==null?"None":r[1])); // floor=4 ceil=6
    }
}
