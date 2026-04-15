// Topological DP over DAG: dp[node][c] = max count of letter c on a path ending at node.
// Kahn's algorithm detects cycles (return null). Time O((V+E)*26), Space O(V*26).
import java.util.*;

public class Solution {
    // Returns largest path value, or empty Optional if a cycle exists (null case).
    static Optional<Integer> largestPathValue(String colors, int[][] edges) {
        int n = colors.length();
        List<List<Integer>> adj = new ArrayList<>();
        for (int i = 0; i < n; i++) adj.add(new ArrayList<>());
        int[] indeg = new int[n];
        for (int[] e : edges) {
            adj.get(e[0]).add(e[1]);
            indeg[e[1]]++;
        }

        int[][] dp = new int[n][26];
        Deque<Integer> q = new ArrayDeque<>();
        for (int i = 0; i < n; i++) if (indeg[i] == 0) q.add(i);

        int processed = 0, ans = 0;
        while (!q.isEmpty()) {
            int u = q.poll();
            processed++;
            int cu = colors.charAt(u) - 'A';
            dp[u][cu]++;
            ans = Math.max(ans, dp[u][cu]);
            for (int v : adj.get(u)) {
                for (int c = 0; c < 26; c++)
                    dp[v][c] = Math.max(dp[v][c], dp[u][c]);
                if (--indeg[v] == 0) q.add(v);
            }
        }
        if (processed < n) return Optional.empty(); // cycle -> null
        return Optional.of(ans);
    }

    public static void main(String[] args) {
        String colors = "ABACA";
        int[][] edges = {{0,1},{0,2},{2,3},{3,4}};
        Optional<Integer> res = largestPathValue(colors, edges);
        System.out.println(res.isPresent() ? res.get() : "null");
    }
}
