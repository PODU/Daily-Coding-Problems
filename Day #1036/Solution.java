// Day 1036: Reconstruct BST from postorder traversal.
// Approach: walk postorder in reverse (root,right,left) using value bounds.
// Time: O(n), Space: O(h) recursion.
public class Solution {
    static class Node { int val; Node left, right; Node(int v){ val = v; } }

    static int idx;
    static Node build(int[] post, long bound) {
        if (idx < 0 || post[idx] < bound) return null;
        Node node = new Node(post[idx--]);
        node.right = build(post, node.val);
        node.left  = build(post, bound);
        return node;
    }

    static void printSideways(Node n, int depth) {
        if (n == null) return;
        printSideways(n.right, depth + 1);
        System.out.println(" ".repeat(depth * 4) + n.val);
        printSideways(n.left, depth + 1);
    }

    public static void main(String[] args) {
        int[] post = {2, 4, 3, 8, 7, 5};
        idx = post.length - 1;
        Node root = build(post, Long.MIN_VALUE);
        System.out.println("Reconstructed BST (rotated 90 deg, root=5):");
        printSideways(root, 0);
    }
}
