// Post-order DFS: compute each subtree sum, tally counts in a HashMap, return most frequent.
// Time O(n), Space O(n).
import java.util.HashMap;
import java.util.Map;

public class Solution {
    static class Node {
        int val;
        Node left, right;
        Node(int v) { val = v; }
        Node(int v, Node l, Node r) { val = v; left = l; right = r; }
    }

    static int dfs(Node node, Map<Integer, Integer> counts) {
        if (node == null) return 0;
        int s = node.val + dfs(node.left, counts) + dfs(node.right, counts);
        counts.merge(s, 1, Integer::sum);
        return s;
    }

    static int mostFrequentSubtreeSum(Node root) {
        Map<Integer, Integer> counts = new HashMap<>();
        dfs(root, counts);
        int best = 0, ans = 0;
        boolean first = true;
        for (Map.Entry<Integer, Integer> e : counts.entrySet()) {
            if (first || e.getValue() > best || (e.getValue() == best && e.getKey() < ans)) {
                best = e.getValue();
                ans = e.getKey();
                first = false;
            }
        }
        return ans;
    }

    public static void main(String[] args) {
        Node root = new Node(5, new Node(2), new Node(-5));
        System.out.println(mostFrequentSubtreeSum(root));
    }
}
