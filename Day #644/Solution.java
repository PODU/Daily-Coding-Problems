// Day 644: Count unival subtrees (all nodes share one value).
// Approach: post-order DFS; a node is unival iff both children are unival and
// their values match the node's. Count as we recurse.
// Time: O(n), Space: O(h).
public class Solution {
    static class Node {
        int val; Node l, r;
        Node(int v) { val = v; }
    }

    static int count = 0;

    static boolean dfs(Node node) {
        if (node == null) return true;
        boolean left = dfs(node.l);
        boolean right = dfs(node.r);
        if (!left || !right) return false;
        if (node.l != null && node.l.val != node.val) return false;
        if (node.r != null && node.r.val != node.val) return false;
        count++;
        return true;
    }

    static int countUnival(Node root) {
        count = 0;
        dfs(root);
        return count;
    }

    public static void main(String[] args) {
        Node root = new Node(0);
        root.l = new Node(1);
        root.r = new Node(0);
        root.r.l = new Node(1);
        root.r.r = new Node(0);
        root.r.l.l = new Node(1);
        root.r.l.r = new Node(1);
        System.out.println(countUnival(root)); // 5
    }
}
