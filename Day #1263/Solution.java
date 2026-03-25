// Day 1263: All root-to-leaf paths in a binary tree.
// DFS carrying the current path. O(n) nodes visited, O(h) recursion + output size.
import java.util.*;

public class Solution {
    static class Node {
        int val; Node left, right;
        Node(int v) { val = v; }
    }

    static void dfs(Node node, List<Integer> path, List<List<Integer>> res) {
        if (node == null) return;
        path.add(node.val);
        if (node.left == null && node.right == null) res.add(new ArrayList<>(path));
        else { dfs(node.left, path, res); dfs(node.right, path, res); }
        path.remove(path.size() - 1);
    }

    static List<List<Integer>> rootToLeaf(Node root) {
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
        System.out.println(rootToLeaf(root));
    }
}
