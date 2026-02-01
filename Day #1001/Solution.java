// Day 1001: Validate a binary search tree.
// Recurse carrying an allowed [low, high] range; left key <= root <= right key.
// O(n) time, O(h) space.
public class Solution {
    static class Node {
        long val;
        Node left, right;
        Node(long v) { val = v; }
        Node(long v, Node l, Node r) { val = v; left = l; right = r; }
    }

    static boolean isBst(Node node, long low, long high) {
        if (node == null) return true;
        if (node.val < low || node.val > high) return false;
        return isBst(node.left, low, node.val) && isBst(node.right, node.val, high);
    }

    public static void main(String[] args) {
        Node valid = new Node(5, new Node(3, new Node(2), new Node(4)),
                                 new Node(8, new Node(6), new Node(9)));
        Node invalid = new Node(5, new Node(6), new Node(8));
        System.out.println(isBst(valid, Long.MIN_VALUE, Long.MAX_VALUE));   // true
        System.out.println(isBst(invalid, Long.MIN_VALUE, Long.MAX_VALUE)); // false
    }
}
