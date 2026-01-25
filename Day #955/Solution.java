// Day 955: evaluate an arithmetic expression binary tree (leaves=ints, nodes=+ - * /).
// Recursive post-order evaluation. Time O(n), Space O(h).
public class Solution {
    static class Node {
        char op; double val; Node left, right;
        Node(double v) { this.val = v; }
        Node(char o, Node l, Node r) { this.op = o; this.left = l; this.right = r; }
    }

    static double eval(Node n) {
        if (n.left == null && n.right == null) return n.val;
        double a = eval(n.left), b = eval(n.right);
        switch (n.op) {
            case '+': return a + b;
            case '-': return a - b;
            case '*': return a * b;
            default:  return a / b;
        }
    }

    public static void main(String[] args) {
        Node root = new Node('*',
            new Node('+', new Node(3), new Node(2)),
            new Node('+', new Node(4), new Node(5)));
        System.out.println((long) eval(root)); // 45
    }
}
