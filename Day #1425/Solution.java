// Day 1425: evaluate an arithmetic expression binary tree (+,-,*,/ internal; ints at leaves).
// Approach: post-order recursion, combine children by operator. O(n) time, O(h) space.
public class Solution {
    static class Node {
        String op;   // operator for internal nodes, null for leaves
        int val;
        Node left, right;
        Node(int v) { val = v; }
        Node(String o, Node l, Node r) { op = o; left = l; right = r; }
    }

    static double eval(Node n) {
        if (n.left == null && n.right == null) return n.val;
        double a = eval(n.left), b = eval(n.right);
        switch (n.op) {
            case "+": return a + b;
            case "-": return a - b;
            case "*": return a * b;
            default:  return a / b;
        }
    }

    public static void main(String[] args) {
        Node root = new Node("*",
            new Node("+", new Node(3), new Node(2)),
            new Node("+", new Node(4), new Node(5)));
        System.out.println((long) eval(root)); // 45
    }
}
