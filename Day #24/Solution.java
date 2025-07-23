// Binary tree locking: each node has a parent pointer and lockedDescendantCount.
// lock/unlock check ancestors (O(h)) + descendant count, then update ancestors (O(h)).
public class Solution {
    static class Node {
        String name;
        Node parent, left, right;
        boolean locked;
        int lockedDescendantCount;

        Node(String name, Node parent) {
            this.name = name;
            this.parent = parent;
        }

        boolean isLocked() { return locked; }

        private boolean canLock() {
            if (lockedDescendantCount > 0) return false;
            for (Node cur = parent; cur != null; cur = cur.parent)
                if (cur.locked) return false;
            return true;
        }

        boolean lock() {
            if (locked) return false;
            if (!canLock()) return false;
            locked = true;
            for (Node cur = parent; cur != null; cur = cur.parent)
                cur.lockedDescendantCount++;
            return true;
        }

        boolean unlock() {
            if (!locked) return false;
            locked = false;
            for (Node cur = parent; cur != null; cur = cur.parent)
                cur.lockedDescendantCount--;
            return true;
        }
    }

    public static void main(String[] args) {
        Node root = new Node("root", null);
        Node node1 = new Node("node1", root);
        Node node2 = new Node("node2", root);
        root.left = node1;
        root.right = node2;
        Node node3 = new Node("node3", node1);
        Node node4 = new Node("node4", node1);
        node1.left = node3;
        node1.right = node4;

        System.out.println("node2.lock() = " + node2.lock());
        System.out.println("root.lock() = " + root.lock());
        System.out.println("node2.unlock() = " + node2.unlock());
        System.out.println("root.lock() = " + root.lock());
    }
}
