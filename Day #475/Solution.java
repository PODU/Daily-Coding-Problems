// Tree locking with parent pointers + per-node lockedDescendantCount.
// lock/unlock are O(h): walk ancestors once to check, once to update counts.
public class Solution {
    static class Node {
        String name;
        Node parent, left, right;
        boolean locked;
        int lockedDescendantCount;

        Node(String n) { name = n; }

        boolean isLocked() { return locked; }

        boolean anyAncestorLocked() {
            for (Node p = parent; p != null; p = p.parent)
                if (p.locked) return true;
            return false;
        }

        boolean lock() {
            if (locked) return false;
            if (lockedDescendantCount > 0) return false; // a descendant is locked
            if (anyAncestorLocked()) return false;       // an ancestor is locked
            locked = true;
            for (Node p = parent; p != null; p = p.parent) p.lockedDescendantCount++;
            return true;
        }

        boolean unlock() {
            if (!locked) return false;
            locked = false;
            for (Node p = parent; p != null; p = p.parent) p.lockedDescendantCount--;
            return true;
        }
    }

    static String cap(boolean b) { return b ? "True" : "False"; }

    static Node child(Node p, Node c, boolean left) {
        if (left) p.left = c; else p.right = c;
        c.parent = p;
        return c;
    }

    public static void main(String[] args) {
        Node n1 = new Node("node1");
        Node n2 = new Node("node2");
        Node n3 = new Node("node3");
        Node n4 = new Node("node4");
        Node n5 = new Node("node5");
        child(n1, n2, true); child(n1, n3, false);
        child(n2, n4, true); child(n2, n5, false);

        System.out.println("lock(node4): " + cap(n4.lock()));     // True
        System.out.println("lock(node2): " + cap(n2.lock()));     // False (descendant locked)
        System.out.println("unlock(node4): " + cap(n4.unlock())); // True
        System.out.println("lock(node2): " + cap(n2.lock()));     // True
        System.out.println("lock(node1): " + cap(n1.lock()));     // False (descendant locked)
        System.out.println("lock(node5): " + cap(n5.lock()));     // False (ancestor locked)
        System.out.println("unlock(node2): " + cap(n2.unlock())); // True
        System.out.println("lock(node1): " + cap(n1.lock()));     // True
    }
}
