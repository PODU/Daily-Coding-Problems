// Day 1501: Most frequent subtree sum.
// Approach: post-order DFS, accumulate subtree sums in a hash map, pick max count.
// Time: O(n), Space: O(n).
import java.util.*;

public class Solution {
    static class Node {
        int val;
        Node left, right;
        Node(int v) { val = v; }
    }

    static int dfs(Node node, Map<Integer,Integer> freq) {
        if (node == null) return 0;
        int sum = node.val + dfs(node.left, freq) + dfs(node.right, freq);
        freq.merge(sum, 1, Integer::sum);
        return sum;
    }

    static List<Integer> mostFrequentSubtreeSum(Node root) {
        Map<Integer,Integer> freq = new HashMap<>();
        dfs(root, freq);
        int best = 0;
        for (int c : freq.values()) best = Math.max(best, c);
        List<Integer> res = new ArrayList<>();
        for (Map.Entry<Integer,Integer> e : freq.entrySet())
            if (e.getValue() == best) res.add(e.getKey());
        Collections.sort(res);
        return res;
    }

    public static void main(String[] args) {
        Node root = new Node(5);
        root.left = new Node(2);
        root.right = new Node(-5);
        List<Integer> res = mostFrequentSubtreeSum(root);
        StringBuilder sb = new StringBuilder();
        for (int i = 0; i < res.size(); i++) {
            if (i > 0) sb.append(" ");
            sb.append(res.get(i));
        }
        System.out.println(sb.toString());
    }
}
