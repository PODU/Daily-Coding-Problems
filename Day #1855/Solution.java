// Day 1855: LCA in a binary tree with parent pointers.
// Two-pointer walk (like linked-list intersection): swap to the other node at root; meet at LCA. O(h) time, O(1) space.
public class Solution {
    static class Node {
        int val;
        Node left, right, parent;
        Node(int v) { val = v; }
    }

    static Node lca(Node p, Node q) {
        Node a = p, b = q;
        while (a != b) {
            a = (a == null) ? q : a.parent;
            b = (b == null) ? p : b.parent;
        }
        return a;
    }

    static Node attach(Node parent, Node child) { if (child != null) child.parent = parent; return child; }

    public static void main(String[] args) {
        Node n1 = new Node(1), n2 = new Node(2), n3 = new Node(3), n4 = new Node(4);
        Node n5 = new Node(5), n6 = new Node(6), n7 = new Node(7), n8 = new Node(8);
        n1.left = attach(n1, n2); n1.right = attach(n1, n3);
        n2.left = attach(n2, n4); n2.right = attach(n2, n5);
        n3.right = attach(n3, n6);
        n5.left = attach(n5, n7); n5.right = attach(n5, n8);

        System.out.println(lca(n7, n8).val); // 5
        System.out.println(lca(n7, n6).val); // 1
        System.out.println(lca(n4, n8).val); // 2
    }
}
