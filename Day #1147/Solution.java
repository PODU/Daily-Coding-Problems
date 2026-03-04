// Day 1147: Locking in a binary tree.
// Node keeps parent + count of locked descendants; lock/unlock walk ancestors. O(h).
public class Solution {
    static class Node {
        Node left, right, parent;
        boolean locked = false;
        int lockedDesc = 0;

        boolean isLocked() { return locked; }

        boolean canLock() {
            if (locked || lockedDesc > 0) return false;
            for (Node a = parent; a != null; a = a.parent) if (a.locked) return false;
            return true;
        }

        boolean lock() {
            if (!canLock()) return false;
            locked = true;
            for (Node a = parent; a != null; a = a.parent) a.lockedDesc++;
            return true;
        }

        boolean unlock() {
            if (!locked) return false;
            locked = false;
            for (Node a = parent; a != null; a = a.parent) a.lockedDesc--;
            return true;
        }
    }

    public static void main(String[] args) {
        Node root = new Node(), l = new Node(), r = new Node(), ll = new Node();
        root.left = l; root.right = r; l.parent = root; r.parent = root;
        l.left = ll; ll.parent = l;
        System.out.println(l.lock());    // true
        System.out.println(ll.lock());   // false
        System.out.println(root.lock()); // false
        System.out.println(l.unlock());  // true
        System.out.println(ll.lock());   // true
    }
}
