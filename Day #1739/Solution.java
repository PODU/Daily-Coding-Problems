// Tree node with parent pointer + lockedDescendants count; lock/unlock walk ancestors O(h).
// isLocked O(1). lock/unlock O(h). Space O(n).
public class Solution {
    static class Node {
        Node parent, left, right;
        boolean locked = false;
        int lockedDescendants = 0;

        Node(Node parent) { this.parent = parent; }

        boolean isLocked() { return locked; }

        boolean anyAncestorLocked() {
            for (Node p = parent; p != null; p = p.parent)
                if (p.locked) return true;
            return false;
        }

        boolean lock() {
            if (locked || lockedDescendants > 0 || anyAncestorLocked()) return false;
            locked = true;
            for (Node p = parent; p != null; p = p.parent) p.lockedDescendants++;
            return true;
        }

        boolean unlock() {
            if (!locked || lockedDescendants > 0 || anyAncestorLocked()) return false;
            locked = false;
            for (Node p = parent; p != null; p = p.parent) p.lockedDescendants--;
            return true;
        }
    }

    public static void main(String[] args) {
        Node root = new Node(null);
        root.left = new Node(root);
        root.right = new Node(root);
        root.left.left = new Node(root.left);
        root.left.right = new Node(root.left);
        Node L = root.left;
        Node LL = root.left.left;

        System.out.println(LL.lock());
        System.out.println(L.lock());
        System.out.println(root.lock());
        System.out.println(LL.unlock());
        System.out.println(L.lock());
        System.out.println(root.lock());
    }
}
