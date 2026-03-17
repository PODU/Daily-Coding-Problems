// Post-order DFS computing subtree sums, count frequencies in a hashmap,
// return sum(s) with max frequency. O(n) time, O(n) space.
import java.util.*;

public class Solution {
    static class Node {
        int val;
        Node left, right;
        Node(int v) { val = v; }
    }

    static Map<Integer, Integer> freq = new HashMap<>();

    static int dfs(Node node) {
        if (node == null) return 0;
        int s = node.val + dfs(node.left) + dfs(node.right);
        freq.merge(s, 1, Integer::sum);
        return s;
    }

    static List<Integer> mostFrequentSubtreeSum(Node root) {
        freq.clear();
        dfs(root);
        int best = 0;
        for (int c : freq.values()) best = Math.max(best, c);
        List<Integer> res = new ArrayList<>();
        for (Map.Entry<Integer, Integer> e : freq.entrySet())
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
            if (i > 0) sb.append(' ');
            sb.append(res.get(i));
        }
        System.out.println(sb.toString());
    }
}
