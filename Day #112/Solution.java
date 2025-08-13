// Day 112: LCA with parent pointers - equalize depths, walk up together. O(h).
public class Solution {
    static class Node {
        int val; Node parent, l, r;
        Node(int v){ val = v; }
    }
    static Node mk(Node p, int v){ Node n = new Node(v); n.parent = p; return n; }
    static int depth(Node n){ int d = 0; while (n != null){ n = n.parent; d++; } return d; }

    static Node lca(Node a, Node b){
        int da = depth(a), db = depth(b);
        while (da > db){ a = a.parent; da--; }
        while (db > da){ b = b.parent; db--; }
        while (a != b){ a = a.parent; b = b.parent; }
        return a;
    }

    public static void main(String[] args){
        Node root = new Node(3);
        root.l = mk(root, 5); root.r = mk(root, 1);
        root.l.l = mk(root.l, 6); root.l.r = mk(root.l, 2);
        root.r.l = mk(root.r, 0); root.r.r = mk(root.r, 8);
        root.l.r.l = mk(root.l.r, 7); root.l.r.r = mk(root.l.r, 4);
        System.out.println(lca(root.l, root.r).val);            // 3
        System.out.println(lca(root.l.r.l, root.l.r.r).val);    // 2
    }
}
