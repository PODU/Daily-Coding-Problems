// DFS tracking depth; record the node value seen at maximum depth. Time O(n), Space O(h).
public class Solution {
    static class Node {
        String val;
        Node left, right;
        Node(String v) { val = v; }
    }

    static int maxDepth = -1;
    static String deepest = null;

    static void dfs(Node node, int depth) {
        if (node == null) return;
        if (depth > maxDepth) { maxDepth = depth; deepest = node.val; }
        dfs(node.left, depth + 1);
        dfs(node.right, depth + 1);
    }

    public static void main(String[] args) {
        Node a = new Node("a");
        a.left = new Node("b");
        a.right = new Node("c");
        a.left.left = new Node("d");
        dfs(a, 0);
        System.out.println(deepest);
    }
}
