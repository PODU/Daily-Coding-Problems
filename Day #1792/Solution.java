// Binary tree max path sum: DFS returning max downward gain; track global max = node + max(0,left) + max(0,right).
// Time O(n), Space O(h).
public class Solution {
    static class Node { int val; Node left, right; Node(int v){ val=v; } }
    static int best;
    static int gain(Node n) {
        if (n == null) return 0;
        int l = Math.max(0, gain(n.left));
        int r = Math.max(0, gain(n.right));
        best = Math.max(best, n.val + l + r);
        return n.val + Math.max(l, r);
    }
    static int maxPathSum(Node root) { best = Integer.MIN_VALUE; gain(root); return best; }
    public static void main(String[] args) {
        Node root = new Node(-10);
        root.left = new Node(9);
        root.right = new Node(20);
        root.right.left = new Node(15);
        root.right.right = new Node(7);
        System.out.println(maxPathSum(root)); // expected 42
    }
}
