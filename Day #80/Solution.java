// Day 80: Return a deepest node of a binary tree via DFS tracking depth.
// Time O(n), Space O(h).
public class Solution {
    static class Node {
        char val; Node left, right;
        Node(char v) { val = v; }
    }

    static int best;
    static char res;

    static void dfs(Node n, int depth) {
        if (n == null) return;
        if (depth > best) { best = depth; res = n.val; }
        dfs(n.left, depth + 1);
        dfs(n.right, depth + 1);
    }

    static char deepestNode(Node root) {
        best = -1; res = 0;
        dfs(root, 0);
        return res;
    }

    public static void main(String[] args) {
        Node a = new Node('a');
        a.left = new Node('b');
        a.right = new Node('c');
        a.left.left = new Node('d');
        System.out.println(deepestNode(a)); // d
    }
}
