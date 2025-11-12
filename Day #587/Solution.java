// Day 587: Binary tree root-to-leaf paths.
// Approach: DFS, accumulate current path, record at leaves. Time O(n), Space O(h).
import java.util.ArrayList;
import java.util.List;

public class Solution {
    static class Node {
        int val;
        Node left, right;
        Node(int v) { val = v; }
    }

    static void dfs(Node node, List<Integer> path, List<List<Integer>> res) {
        if (node == null) return;
        path.add(node.val);
        if (node.left == null && node.right == null) {
            res.add(new ArrayList<>(path));
        } else {
            dfs(node.left, path, res);
            dfs(node.right, path, res);
        }
        path.remove(path.size() - 1);
    }

    static List<List<Integer>> rootToLeafPaths(Node root) {
        List<List<Integer>> res = new ArrayList<>();
        dfs(root, new ArrayList<>(), res);
        return res;
    }

    public static void main(String[] args) {
        Node root = new Node(1);
        root.left = new Node(2);
        root.right = new Node(3);
        root.right.left = new Node(4);
        root.right.right = new Node(5);

        System.out.println(rootToLeafPaths(root));
    }
}
