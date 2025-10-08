// Root-to-leaf path sum via DFS subtracting node values; leaf checks remainder==0. O(n) time, O(h) space.
public class Solution {
    static class Node {
        int val; Node left, right;
        Node(int v) { val = v; }
    }

    static boolean hasPathSum(Node root, int k) {
        if (root == null) return false;
        if (root.left == null && root.right == null) return k - root.val == 0;
        return hasPathSum(root.left, k - root.val) || hasPathSum(root.right, k - root.val);
    }

    public static void main(String[] args) {
        Node root = new Node(8);
        root.left = new Node(4);
        root.right = new Node(13);
        root.left.left = new Node(2);
        root.left.right = new Node(6);
        root.right.right = new Node(19);
        System.out.println(hasPathSum(root, 18));
    }
}
