// Day 482: BST range sum [a,b] inclusive via DFS with pruning.
// Skip left subtree if node<a, skip right if node>b. Time O(n) worst, O(k+h) typical; Space O(h).
public class Solution {
    static class Node {
        int val;
        Node left, right;
        Node(int v) { val = v; }
    }

    static int rangeSum(Node root, int a, int b) {
        if (root == null) return 0;
        if (root.val < a) return rangeSum(root.right, a, b);
        if (root.val > b) return rangeSum(root.left, a, b);
        return root.val + rangeSum(root.left, a, b) + rangeSum(root.right, a, b);
    }

    public static void main(String[] args) {
        Node root = new Node(5);
        root.left = new Node(3);
        root.right = new Node(8);
        root.left.left = new Node(2);
        root.left.right = new Node(4);
        root.right.left = new Node(6);
        root.right.right = new Node(10);
        System.out.println(rangeSum(root, 4, 9)); // 23
    }
}
