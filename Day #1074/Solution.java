// LCA with parent pointers: get depths via parent walk, level-up deeper node, advance both until equal. O(h) time O(1) space.
public class Solution {
    static class Node {
        int val;
        Node left, right, parent;
        Node(int v) { val = v; }
    }

    static Node link(Node par, Node child, boolean isLeft) {
        child.parent = par;
        if (isLeft) par.left = child; else par.right = child;
        return child;
    }

    static int depth(Node n) {
        int d = 0;
        while (n.parent != null) { n = n.parent; d++; }
        return d;
    }

    static Node lca(Node a, Node b) {
        int da = depth(a), db = depth(b);
        while (da > db) { a = a.parent; da--; }
        while (db > da) { b = b.parent; db--; }
        while (a != b) { a = a.parent; b = b.parent; }
        return a;
    }

    public static void main(String[] args) {
        Node n3 = new Node(3);
        Node n5 = link(n3, new Node(5), true);
        Node n1 = link(n3, new Node(1), false);
        Node n6 = link(n5, new Node(6), true);
        Node n2 = link(n5, new Node(2), false);
        link(n1, new Node(0), true);
        link(n1, new Node(8), false);
        Node n7 = link(n2, new Node(7), true);
        Node n4 = link(n2, new Node(4), false);

        System.out.println("LCA(7,4) = " + lca(n7, n4).val);
        System.out.println("LCA(6,4) = " + lca(n6, n4).val);
        System.out.println("LCA(7,1) = " + lca(n7, n1).val);
    }
}
