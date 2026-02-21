// Day 1112 - Minimum root-to-leaf path sum (return the path)
// Approach: DFS, track best leaf path by sum. Time: O(n), Space: O(h).
import java.util.*;

public class Solution {
    static class Node {
        int val; Node left, right;
        Node(int v) { val = v; }
        Node(int v, Node l, Node r) { val = v; left = l; right = r; }
    }

    static int bestSum = Integer.MAX_VALUE;
    static List<Integer> bestPath = new ArrayList<>();

    static void dfs(Node node, List<Integer> path, int s) {
        if (node == null) return;
        path.add(node.val);
        s += node.val;
        if (node.left == null && node.right == null) {
            if (s < bestSum) { bestSum = s; bestPath = new ArrayList<>(path); }
        } else {
            dfs(node.left, path, s);
            dfs(node.right, path, s);
        }
        path.remove(path.size() - 1);
    }

    public static void main(String[] args) {
        Node root = new Node(10,
            new Node(5, null, new Node(2)),
            new Node(5, null, new Node(1, new Node(-1), null)));
        dfs(root, new ArrayList<>(), 0);
        System.out.println(bestPath + ", which has sum " + bestSum);
    }
}
