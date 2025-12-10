// Day 724: Evaluate an arithmetic expression binary tree.
// Approach: Post-order recursion; leaves are ints, internal nodes are operators.
// Time: O(n), Space: O(h).
public class Solution {
    static class Node {
        char op;       // 0 for leaf
        int val;
        Node left, right;
        Node(int v) { this.val = v; }
        Node(char o, Node l, Node r) { this.op = o; this.left = l; this.right = r; }
    }

    static double eval(Node root) {
        if (root.left == null && root.right == null) return root.val;
        double l = eval(root.left), r = eval(root.right);
        switch (root.op) {
            case '+': return l + r;
            case '-': return l - r;
            case '*': return l * r;
            case '/': return l / r;
        }
        return 0;
    }

    public static void main(String[] args) {
        Node tree = new Node('*',
            new Node('+', new Node(3), new Node(2)),
            new Node('+', new Node(4), new Node(5)));
        System.out.println((long) eval(tree)); // 45
    }
}
