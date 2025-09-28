// Range Sum of BST. Pruned DFS using BST property. Time O(n) worst, Space O(h).
public class Solution {
    static class Node {
        int val;
        Node left, right;
        Node(int v) { val = v; }
    }

    static int rangeSum(Node root, int a, int b) {
        if (root == null) return 0;
        int s = 0;
        if (root.val >= a && root.val <= b) s += root.val;
        if (root.val > a) s += rangeSum(root.left, a, b);
        if (root.val < b) s += rangeSum(root.right, a, b);
        return s;
    }

    public static void main(String[] args) {
        Node root = new Node(5);
        root.left = new Node(3);
        root.left.left = new Node(2);
        root.left.right = new Node(4);
        root.right = new Node(8);
        root.right.left = new Node(6);
        root.right.right = new Node(10);
        System.out.println(rangeSum(root, 4, 9));
    }
}
