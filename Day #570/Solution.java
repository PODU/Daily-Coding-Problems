// Subtree check: at each node of s, test sameTree(node, t).
// Time: O(|s|*|t|) worst case. Space: O(height). (Optimal O(|s|+|t|) via
// tree serialization + KMP substring search; recursive version implemented.)
public class Solution {
    static class Node {
        int val;
        Node left, right;
        Node(int v) { val = v; }
    }

    static boolean sameTree(Node a, Node b) {
        if (a == null && b == null) return true;
        if (a == null || b == null) return false;
        return a.val == b.val && sameTree(a.left, b.left) && sameTree(a.right, b.right);
    }

    static boolean isSubtree(Node s, Node t) {
        if (s == null) return false;
        if (sameTree(s, t)) return true;
        return isSubtree(s.left, t) || isSubtree(s.right, t);
    }

    public static void main(String[] args) {
        // s:        3
        //          / \
        //         4   5
        //        / \
        //       1   2
        Node s = new Node(3);
        s.left = new Node(4);
        s.right = new Node(5);
        s.left.left = new Node(1);
        s.left.right = new Node(2);

        Node t = new Node(4);
        t.left = new Node(1);
        t.right = new Node(2);

        Node t2 = new Node(4);
        t2.left = new Node(1);
        t2.right = new Node(0);

        System.out.println(isSubtree(s, t));
        System.out.println(isSubtree(s, t2));
    }
}
