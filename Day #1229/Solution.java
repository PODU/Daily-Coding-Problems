// Count unival subtrees via post-order DFS; node is unival if both children unival and values match.
// Time: O(n), Space: O(h) recursion.
public class Solution {
    static class Node {
        int val; Node left, right;
        Node(int v) { val = v; }
    }

    static int count = 0;

    // returns true if subtree rooted here is unival; increments count.
    static boolean dfs(Node n) {
        if (n == null) return true;
        boolean l = dfs(n.left);
        boolean r = dfs(n.right);
        if (!l || !r) return false;
        if (n.left != null && n.left.val != n.val) return false;
        if (n.right != null && n.right.val != n.val) return false;
        count++;
        return true;
    }

    public static void main(String[] args) {
        Node root = new Node(0);
        root.left = new Node(1);
        root.right = new Node(0);
        root.right.left = new Node(1);
        root.right.right = new Node(0);
        root.right.left.left = new Node(1);
        root.right.left.right = new Node(1);
        dfs(root);
        System.out.println(count);
    }
}
