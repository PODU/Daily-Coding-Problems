// Height-balanced check via bottom-up recursion returning height, -1 sentinel = unbalanced.
// Time O(n), Space O(h).
public class Solution {
    static class Node {
        int val;
        Node left, right;
        Node(int v) { val = v; }
    }

    static int height(Node n) {
        if (n == null) return 0;
        int lh = height(n.left);
        if (lh == -1) return -1;
        int rh = height(n.right);
        if (rh == -1) return -1;
        if (Math.abs(lh - rh) > 1) return -1;
        return Math.max(lh, rh) + 1;
    }

    static boolean isBalanced(Node root) {
        return height(root) != -1;
    }

    public static void main(String[] args) {
        // Balanced tree
        Node a = new Node(1);
        a.left = new Node(2);
        a.right = new Node(3);
        a.left.left = new Node(4);

        // Unbalanced left-leaning chain 1 -> 2 -> 3 -> 4
        Node b = new Node(1);
        b.left = new Node(2);
        b.left.left = new Node(3);
        b.left.left.left = new Node(4);

        System.out.println(isBalanced(a));
        System.out.println(isBalanced(b));
    }
}
