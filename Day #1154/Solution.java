// Day 1154: Minimum root-to-leaf path sum.
// DFS tracking running sum/path, keep best at leaves. O(n) time, O(h) space.
import java.util.*;

public class Solution {
    static class Node { int val; Node left, right; Node(int v) { val = v; } }

    static int best = Integer.MAX_VALUE;
    static List<Integer> bestPath = new ArrayList<>();

    static void dfs(Node n, List<Integer> path, int sum) {
        if (n == null) return;
        path.add(n.val); sum += n.val;
        if (n.left == null && n.right == null) {
            if (sum < best) { best = sum; bestPath = new ArrayList<>(path); }
        } else {
            dfs(n.left, path, sum);
            dfs(n.right, path, sum);
        }
        path.remove(path.size() - 1);
    }

    public static void main(String[] args) {
        Node root = new Node(10);
        root.left = new Node(5); root.right = new Node(5);
        root.left.right = new Node(2);
        root.right.right = new Node(1);
        root.right.right.left = new Node(-1);
        dfs(root, new ArrayList<>(), 0);
        System.out.println(bestPath + ", which has sum " + best); // [10, 5, 1, -1], which has sum 15
    }
}
