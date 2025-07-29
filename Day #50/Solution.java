// Day 50: Evaluate arithmetic expression binary tree via post-order recursion.
// Time: O(n), Space: O(h).
public class Solution {
    static class Node {
        char op;      // 0 for leaf
        double val;
        Node left, right;
        Node(double v) { val = v; }
        Node(char o, Node l, Node r) { op = o; left = l; right = r; }
    }

    static double eval(Node n) {
        if (n.op == 0) return n.val;
        double a = eval(n.left), b = eval(n.right);
        switch (n.op) {
            case '+': return a + b;
            case '-': return a - b;
            case '*': return a * b;
            case '/': return a / b;
        }
        return 0;
    }

    public static void main(String[] args) {
        Node root = new Node('*',
            new Node('+', new Node(3), new Node(2)),
            new Node('+', new Node(4), new Node(5)));
        System.out.println((long) eval(root));
    }
}
