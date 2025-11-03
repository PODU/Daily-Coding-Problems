// LCA with parent pointers: equalize depths, then walk both up together.
// Time O(h), Space O(1).
public class Solution {
    static class Node {
        int val;
        Node parent, left, right;
        Node(int v) { val = v; }
    }

    static int depth(Node n) {
        int d = 0;
        while (n != null) { d++; n = n.parent; }
        return d;
    }

    static Node lca(Node a, Node b) {
        int da = depth(a), db = depth(b);
        while (da > db) { a = a.parent; da--; }
        while (db > da) { b = b.parent; db--; }
        while (a != b) { a = a.parent; b = b.parent; }
        return a;
    }

    static Node child(Node parent, Node c) {
        if (c != null) c.parent = parent;
        return c;
    }

    public static void main(String[] args) {
        Node n3 = new Node(3), n5 = new Node(5), n1 = new Node(1);
        Node n6 = new Node(6), n2 = new Node(2), n0 = new Node(0), n8 = new Node(8);
        Node n7 = new Node(7), n4 = new Node(4);

        n3.left = child(n3, n5);
        n3.right = child(n3, n1);
        n5.left = child(n5, n6);
        n5.right = child(n5, n2);
        n1.left = child(n1, n0);
        n1.right = child(n1, n8);
        n2.left = child(n2, n7);
        n2.right = child(n2, n4);

        System.out.println(lca(n6, n4).val);
        System.out.println(lca(n6, n8).val);
    }
}
