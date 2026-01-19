// Locking binary tree: each node tracks lockedDescendants count; lock/unlock check
// ancestors + descendant count. All ops O(h) where h is tree height.
public class Solution {
    static class Node {
        Node parent, left, right;
        boolean locked = false;
        int lockedDescendants = 0;

        boolean isLocked() { return locked; }

        private boolean anyAncestorLocked() {
            for (Node p = parent; p != null; p = p.parent)
                if (p.locked) return true;
            return false;
        }

        boolean lock() {
            if (locked) return false;
            if (lockedDescendants > 0) return false;
            if (anyAncestorLocked()) return false;
            locked = true;
            for (Node p = parent; p != null; p = p.parent) p.lockedDescendants++;
            return true;
        }

        boolean unlock() {
            if (!locked) return false;
            locked = false;
            for (Node p = parent; p != null; p = p.parent) p.lockedDescendants--;
            return true;
        }
    }

    public static void main(String[] args) {
        Node root = new Node(), a = new Node(), b = new Node(), c = new Node(), d = new Node();
        root.left = a; root.right = b;
        a.parent = root; b.parent = root;
        a.left = c; a.right = d;
        c.parent = a; d.parent = a;

        System.out.println("lock c (leaf)      -> " + c.lock()   + " (expect true)");
        System.out.println("lock a (ancestor)  -> " + a.lock()   + " (expect false)");
        System.out.println("lock root          -> " + root.lock()+ " (expect false)");
        System.out.println("unlock c           -> " + c.unlock() + " (expect true)");
        System.out.println("lock a             -> " + a.lock()   + " (expect true)");
        System.out.println("lock c (desc lock) -> " + c.lock()   + " (expect false)");
        System.out.println("unlock a           -> " + a.unlock() + " (expect true)");
    }
}
