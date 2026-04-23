// Day 1411: Check if tree t is a subtree of tree s.
// Approach: recursive DFS — for each node of s try exact-match with t. O(|s|*|t|) time, O(h) space.
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
        Node s = new Node(3);
        s.left = new Node(4);
        s.right = new Node(5);
        s.left.left = new Node(1);
        s.left.right = new Node(2);

        Node t = new Node(4);
        t.left = new Node(1);
        t.right = new Node(2);

        System.out.println(isSubtree(s, t) ? "true" : "false");
    }
}
