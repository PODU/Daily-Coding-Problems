// Evaluate arithmetic expression tree: recurse, combining children by operator
// at each internal node; leaves are integers. Time O(n), Space O(h) recursion.
public class Solution {
    static class Node {
        boolean isLeaf;
        long val;
        char op;
        Node left, right;
        Node(long v) { isLeaf = true; val = v; }
        Node(char o, Node l, Node r) { isLeaf = false; op = o; left = l; right = r; }
    }

    static long eval(Node n) {
        if (n.isLeaf) return n.val;
        long a = eval(n.left), b = eval(n.right);
        switch (n.op) {
            case '+': return a + b;
            case '-': return a - b;
            case '*': return a * b;
            case '/': return a / b;
        }
        return 0;
    }

    public static void main(String[] args) {
        Node left = new Node('+', new Node(3), new Node(2));
        Node right = new Node('+', new Node(4), new Node(5));
        Node root = new Node('*', left, right);
        System.out.println(eval(root));
    }
}
